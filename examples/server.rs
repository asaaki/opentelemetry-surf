//! Example server for testing (copied from opentelemetry-tide)

use async_std::task;
use opentelemetry_semantic_conventions::resource;
use opentelemetry_tide::OpenTelemetryTracingMiddleware;
use std::time::Duration;

mod shared;

const SVC_NAME: &str = env!("CARGO_CRATE_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tide::log::start();
    shared::init_global_propagator();

    let tags = [
        resource::SERVICE_VERSION.string(VERSION),
        resource::SERVICE_INSTANCE_ID.string("backend-123"),
        resource::PROCESS_EXECUTABLE_PATH.string(std::env::current_exe().unwrap().display().to_string()),
        resource::PROCESS_PID.string(std::process::id().to_string()),
    ];

    let (tracer, _uninstall) = opentelemetry_jaeger::new_pipeline()
        .with_service_name(SVC_NAME)
        .with_tags(tags.iter().map(ToOwned::to_owned))
        .install()
        .expect("pipeline install failure");

    let mut app = tide::new();
    app.with(OpenTelemetryTracingMiddleware::new(tracer.clone()));
    app.at("/").get(|_| async move {
        task::sleep(Duration::from_millis(9)).await;
        Ok("Hello, OpenTelemetry!")
    });
    app.listen("127.0.0.1:3000").await?;

    Ok(())
}
