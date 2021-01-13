//! Example client for testing distributed tracing
//!
//! Start the server example first, then call this client.

use opentelemetry::sdk::trace::Tracer;
use opentelemetry::KeyValue;
use opentelemetry_jaeger::Uninstall;
use opentelemetry_surf::OpenTelemetryTracingMiddleware;

const SVC_NAME: &str = env!("CARGO_CRATE_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
include!(concat!(env!("OUT_DIR"), "/build_vars.rs"));

mod shared;

#[async_std::main]
async fn main() -> std::result::Result<(), http_types::Error> {
    femme::with_level(femme::LevelFilter::Info);
    shared::init_global_propagator();

    let (tracer, _uninstall) = pipeline();
    let otel_mw = OpenTelemetryTracingMiddleware::new(tracer);
    let client = create_client().with(otel_mw);

    let res = client.get("http://localhost:3000/").recv_string().await?;
    dbg!(res);
    Ok(())
}

fn tags() -> Vec<KeyValue> {
    use opentelemetry_semantic_conventions::resource;

    vec![
        resource::SERVICE_VERSION.string(VERSION),
        resource::SERVICE_INSTANCE_ID.string("client-42"),
        resource::PROCESS_EXECUTABLE_PATH.string(std::env::current_exe().unwrap().display().to_string()),
        resource::PROCESS_PID.string(std::process::id().to_string()),
        KeyValue::new("process.executable.profile", PROFILE),
    ]
}

fn pipeline() -> (Tracer, Uninstall) {
    opentelemetry_jaeger::new_pipeline()
        .with_service_name(SVC_NAME)
        .with_tags(tags())
        .install()
        .expect("pipeline install failure")
}

// more custom http client setup: use isahc with metrics enabled
fn create_client() -> surf::Client {
    use http_client::isahc::IsahcClient;
    use isahc::config::Configurable;

    let isahc = isahc::HttpClient::builder()
        .default_headers(&[("user-agent", "surf/isahc (with request metrics)")])
        .metrics(true)
        .build()
        .expect("isahc client could no be created");
    let http_client = IsahcClient::from_client(isahc);
    surf::Client::with_http_client(http_client)
}
