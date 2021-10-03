<h1 align="center"><img src="https://raw.githubusercontent.com/asaaki/opentelemetry-surf/main/.assets/opentelemetry-surf-logo.svg" width=128 height=128><br>opentelemetry-surf</h1>
<div align="center"><strong>

[OpenTelemetry] integration for [Surf]

</strong></div><br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/opentelemetry-surf">
    <img src="https://img.shields.io/crates/v/opentelemetry-surf.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- docs.rs -->
  <a href="https://docs.rs/opentelemetry-surf">
    <img src="https://img.shields.io/badge/docs.rs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
    <!-- <img src="https://docs.rs/opentelemetry-surf/badge.svg"
      alt="docs.rs docs" /> -->
  </a>
  <!-- CI -->
  <a href="https://crates.io/crates/opentelemetry-surf">
    <img src="https://img.shields.io/github/workflow/status/asaaki/opentelemetry-surf/CI/main?style=flat-square"
      alt="CI status" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/opentelemetry-surf">
    <img src="https://img.shields.io/crates/d/opentelemetry-surf.svg?style=flat-square"
      alt="Download" />
  </a>
</div>

Add OpenTelemetry tracing support to your [Surf] clients.
Be part of the new observability movement!

## Notes

* It is heavily inspired by [opentelemetry-tide] crate; _surf_ and _tide_ share very similar middleware structure.
  Thank you, dear [@http-rs] folks! 🙇🏻‍♂️
* It only implements very basic request tracing on the middleware layer.
  If something is missing, please open an issue and describe your desired feature or create a PR with a change.
* It can record http request/response life cycle events when used with isahc and its metrics feature enabled.
* You probably do not want to use it in production. 🤷

## How to use

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

## Code example

### `Cargo.toml`

```toml
async-std = { version = "1.10", features = ["attributes"] }
opentelemetry = { version = "0.16", features = ["rt-async-std"] }
opentelemetry-jaeger = { version = "0.15", features = ["rt-async-std"] }
opentelemetry-surf = "0.5"
```

### `client.rs`

```rust
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

## Cargo Features

|            flag | description |
| --------------: | :---------- |
| `isahc-metrics` | enables more details when using a custom ishac client configuration, see `examples/client/metrics.rs` for details |

## Safety

This crate uses ``#![forbid(unsafe_code)]`` to ensure everything is implemented in 100% Safe Rust.

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>



<!-- links -->
[OpenTelemetry]: https://crates.io/crates/opentelemetry
[Surf]: https://crates.io/crates/surf
[opentelemetry-tide]: https://crates.io/crates/opentelemetry-tide
[@http-rs]: https://github.com/http-rs
