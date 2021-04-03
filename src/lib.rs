/*!
Add OpenTelemetry tracing support to your [Surf] clients.
Be part of the new observability movement!

# Notes

* It is heavily inspired by [opentelemetry-tide] crate; _surf_ and _tide_ share very similar middleware structure.
  Thank you, dear [@http-rs] folks! 🙇🏻‍♂️
* It only implements very basic request tracing on the middleware layer.
  If something is missing, please open an issue and describe your desired feature or create a PR with a change.
* It can record http request/response life cycle events when used with isahc and its metrics feature enabled.
* You probably do not want to use it in production. 🤷

# How to use

```shell
# Run jaeger in background
docker run -d \
  -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p14268:14268 \
  jaegertracing/all-in-one:latest


# Run simple client example with tracing middleware
cargo run --example simple

# Run metrics client example (uses isahc with metrics enabled)
cargo run --example metrics --features isahc-metrics

# Open browser and view the traces
firefox http://localhost:16686/
```

![example jaeger trace](https://raw.githubusercontent.com/asaaki/opentelemetry-surf/main/.assets/jaeger-trace.png)

# Code example

## `Cargo.toml`

```toml
async-std = { version = "1.9", features = ["attributes"] }
opentelemetry = { version = "0.13", features = ["async-std", "rt-async-std"] }
opentelemetry-jaeger = { version = "0.12", features = ["async-std"] }
opentelemetry-surf = "0.2"
```

## `client.rs`

```rust,no_run
#[async_std::main]
async fn main() -> surf::Result<()> {
    let tracer = opentelemetry_jaeger::new_pipeline().install_batch(opentelemetry::runtime::AsyncStd)?;
    let otel_mw = opentelemetry_surf::OpenTelemetryTracingMiddleware::new(tracer);
    let client = surf::client().with(otel_mw);
    let res = client.get("https://httpbin.org/get").await?;
    dbg!(res);

    opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}
```

# Cargo Features

|            flag | description |
| --------------: | :---------- |
| `isahc-metrics` | enables more details when using a custom ishac client configuration, see `examples/client/metrics.rs` for details |

# Safety

This crate uses ``#![forbid(unsafe_code)]`` to ensure everything is implemented in 100% Safe Rust.

<!-- links -->
[Surf]: https://crates.io/crates/surf
[opentelemetry-tide]: https://crates.io/crates/opentelemetry-tide
[@http-rs]: https://github.com/http-rs
*/

// !!! GENERATE README WITH: cargo readme > README.md

#![forbid(unsafe_code)]
#![cfg_attr(feature = "docs", feature(doc_cfg))]
#![deny(missing_docs)]
#![deny(unused_imports)]
#![deny(missing_debug_implementations)]
#![warn(clippy::expect_used)]
#![deny(clippy::unwrap_used)]
#![deny(unused_results)]
#![doc(
    test(attr(allow(unused_variables), deny(warnings))),
    html_favicon_url = "https://raw.githubusercontent.com/asaaki/opentelemetry-surf/main/.assets/favicon.ico",
    html_logo_url = "https://raw.githubusercontent.com/asaaki/opentelemetry-surf/main/.assets/docs.png"
)]

use http_types::headers::{HeaderName, HeaderValue};
use kv_log_macro as log;
use opentelemetry::{
    global,
    trace::{FutureExt, Span, SpanKind, StatusCode, TraceContextExt, Tracer},
    Context,
};
use opentelemetry_semantic_conventions::{resource, trace};
use std::collections::HashMap;
use std::convert::TryFrom;
use surf::middleware::{Middleware, Next};
use surf::{http::Version, Client, Request, Response};
use url::Url;

const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The middleware struct to be used in tide
#[derive(Default, Clone, Debug)]
pub struct OpenTelemetryTracingMiddleware<T: Tracer> {
    tracer: T,
}

impl<T: Tracer> OpenTelemetryTracingMiddleware<T> {
    /**
    Instantiate the middleware

    # Examples

    ```rust,no_run
    # #[async_std::main]
    # async fn main() -> surf::Result<()> {
    let tracer = opentelemetry_jaeger::new_pipeline().install_batch(opentelemetry::runtime::AsyncStd)?;
    let otel_mw = opentelemetry_surf::OpenTelemetryTracingMiddleware::new(tracer);
    let client = surf::client().with(otel_mw);
    let res = client.get("https://httpbin.org/get").await?;
    dbg!(res);
    Ok(())
    # }
    ```
    */
    pub fn new(tracer: T) -> Self {
        Self { tracer }
    }
}

#[surf::utils::async_trait]
impl<T: Tracer + Send + Sync> Middleware for OpenTelemetryTracingMiddleware<T> {
    async fn handle(&self, mut req: Request, client: Client, next: Next<'_>) -> Result<Response, http_types::Error> {
        // if request object already has some tracing headers, use them
        // (maybe another middleware or a request builder have injected them)
        let mut req_headers = HashMap::new();
        for (k, v) in req.iter() {
            let _ = req_headers.insert(k.to_string(), v.last().to_string());
        }
        let parent_cx = global::get_text_map_propagator(|propagator| propagator.extract(&req_headers));
        drop(req_headers);

        let method = req.method();
        let url = req.url().clone();

        let mut attributes = Vec::with_capacity(11); // 7 required and 4 optional values
        attributes.push(resource::TELEMETRY_SDK_NAME.string(CRATE_NAME));
        attributes.push(resource::TELEMETRY_SDK_VERSION.string(VERSION));
        attributes.push(resource::TELEMETRY_SDK_LANGUAGE.string("rust"));
        attributes.push(trace::HTTP_METHOD.string(method.to_string()));
        attributes.push(trace::HTTP_SCHEME.string(url.scheme().to_owned()));
        attributes.push(trace::HTTP_URL.string(url.to_string()));
        attributes.push(trace::HTTP_TARGET.string(http_target(&url)));

        if let Some(host) = url.host_str() {
            attributes.push(trace::HTTP_HOST.string(host.to_owned()));
        }

        if let Some(domain) = url.domain() {
            attributes.push(trace::HTTP_SERVER_NAME.string(domain.to_owned()));
        }

        if let Some(port) = url.port_or_known_default() {
            attributes.push(trace::NET_HOST_PORT.i64(port.into()));
        }

        if let Some(len) = req.len().and_then(|len| i64::try_from(len).ok()) {
            attributes.push(trace::HTTP_REQUEST_CONTENT_LENGTH.i64(len));
        }

        let mut span_builder = self
            .tracer
            .span_builder(&format!("{} {}", method, url))
            .with_kind(SpanKind::Client)
            .with_attributes(attributes);

        // make sure our span can be connected to a currently open/active (remote) trace if existing
        if let Some(remote_span_ctx) = parent_cx.remote_span_context() {
            if remote_span_ctx.is_remote() {
                span_builder = span_builder.with_parent_context(parent_cx.clone());
            }
        }

        let span = span_builder.start(&self.tracer);
        span.add_event("request.started".to_owned(), vec![]);
        let cx = &Context::current_with_span(span);

        // (force) set all headers to ensure current span info will be sent
        let mut injector = HashMap::new();
        global::get_text_map_propagator(|propagator| propagator.inject_context(&cx, &mut injector));

        for (k, v) in injector {
            let header_name = HeaderName::from_bytes(k.clone().into_bytes());
            let header_value = HeaderValue::from_bytes(v.clone().into_bytes());
            if let (Ok(name), Ok(value)) = (header_name, header_value) {
                let _ = req.insert_header(name, value);
            } else {
                eprintln!("Could not compose header for pair: ({}, {})", k, v);
            }
        }

        // for event tracing the request lifecycle events of isahc
        #[cfg(feature = "isahc")]
        let req_start = std::time::SystemTime::now();

        // MAKE THE REQUEST!
        let mut res = next.run(req, client).with_context(cx.clone()).await?;

        let span = cx.span();
        span.add_event("request.completed".to_owned(), vec![]);

        span.set_status(span_status(res.status()), "".to_string());
        span.set_attribute(trace::HTTP_STATUS_CODE.i64(u16::from(res.status()).into()));

        if let Some(len) = res.len().and_then(|len| i64::try_from(len).ok()) {
            span.set_attribute(trace::HTTP_RESPONSE_CONTENT_LENGTH.i64(len));
        }

        if let Some(version) = res.version() {
            span.set_attribute(trace::HTTP_FLAVOR.string(http_version_str(version)));
        }

        // NOTE: surf does not allow access to lower level client data
        // if let Some(addr) = ???.local_addr().and_then(socket_str_to_ip) {
        //     attributes.push(trace::HTTP_CLIENT_IP.string(addr.to_string()));
        // }

        // NOTE: surf does not allow access to lower level client data
        // if let Some(addr) = ???.peer_addr().and_then(socket_str_to_ip) {
        //     attributes.push(trace::NET_PEER_IP.string(addr.to_string()));
        // }

        // write trace info to response, so it can be picked up by downstream services
        let mut injector = HashMap::new();
        global::get_text_map_propagator(|propagator| propagator.inject_context(&cx, &mut injector));

        for (k, v) in injector {
            let header_name = HeaderName::from_bytes(k.clone().into_bytes());
            let header_value = HeaderValue::from_bytes(v.clone().into_bytes());
            if let (Ok(name), Ok(value)) = (header_name, header_value) {
                res.insert_header(name, value);
            } else {
                log::error!("Could not compose header for pair: ({}, {})", k, v);
            }
        }

        span.add_event("request.finished".to_owned(), vec![]);

        #[cfg(feature = "isahc-metrics")]
        if let Some(metrics) = &res.ext::<isahc::Metrics>() {
            use opentelemetry::KeyValue;

            span.add_event_with_timestamp("request_start".into(), req_start, vec![]);
            span.add_event_with_timestamp(
                "name_lookup".into(),
                req_start + metrics.name_lookup_time(),
                vec![KeyValue::new(
                    "name_lookup_time",
                    format_duration(metrics.name_lookup_time()),
                )],
            );
            span.add_event_with_timestamp(
                "secure_connect".into(),
                req_start + metrics.secure_connect_time(),
                vec![KeyValue::new(
                    "secure_connect_time",
                    format_duration(metrics.secure_connect_time()),
                )],
            );
            span.add_event_with_timestamp(
                "connect".into(),
                req_start + metrics.connect_time(),
                vec![KeyValue::new("connect_time", format_duration(metrics.connect_time()))],
            );
            span.add_event_with_timestamp(
                "transfer_start".into(),
                req_start + metrics.transfer_start_time(),
                vec![KeyValue::new(
                    "transfer_start_time",
                    format_duration(metrics.transfer_start_time()),
                )],
            );
            span.add_event_with_timestamp(
                "transfer_end".into(),
                req_start + metrics.total_time(),
                vec![
                    KeyValue::new("transfer_time", format_duration(metrics.transfer_time())),
                    KeyValue::new("total_time", format_duration(metrics.total_time())),
                ],
            );
            span.add_event_with_timestamp(
                "redirect".into(),
                req_start + metrics.redirect_time(),
                vec![KeyValue::new("redirect_time", format_duration(metrics.redirect_time()))],
            );
        };

        Ok(res)
    }
}

#[cfg(feature = "isahc-metrics")]
fn format_duration(duration: std::time::Duration) -> String {
    let ns = duration.as_nanos();
    if ns >= 1_000_000_000 {
        // seconds
        format!(
            "{}.{:03}s",
            ns / 1_000_000_000,
            ns.rem_euclid(1_000_000_000) / 1_000_000
        )
    } else if ns >= 1_000_000 {
        // ms
        format!("{}.{:03}ms", ns / 1_000_000, ns.rem_euclid(1_000_000) / 1_000)
    } else if ns >= 1_000 {
        // us
        format!("{}.{:03}us", ns / 1_000, ns.rem_euclid(1_000))
    } else {
        // ns
        format!("{}ns", ns)
    }
}

#[inline]
fn http_version_str(version: Version) -> &'static str {
    use Version::*;
    match version {
        Http0_9 => "0.9",
        Http1_0 => "1.0",
        Http1_1 => "1.1",
        Http2_0 => "2.0",
        Http3_0 => "3.0",
        _ => "unknown",
    }
}

#[inline]
fn http_target(url: &Url) -> String {
    let mut target = String::from(url.path());
    if let Some(q) = url.query() {
        target.push('?');
        target.push_str(q)
    }
    if let Some(f) = url.fragment() {
        target.push('#');
        target.push_str(f);
    }
    target
}

// #[inline]
// fn socket_str_to_ip(socket: &str) -> Option<IpAddr> {
//     SocketAddr::from_str(socket).ok().map(|s| s.ip())
// }

#[inline]
fn span_status(http_status: surf::StatusCode) -> StatusCode {
    match http_status as u16 {
        100..=399 => StatusCode::Ok,
        400..=599 => StatusCode::Error,
        _ => StatusCode::Unset,
    }
}
