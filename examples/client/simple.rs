#[async_std::main]
async fn main() -> surf::Result<()> {
    let _tracer = opentelemetry_jaeger::new_pipeline().install_batch(opentelemetry::runtime::AsyncStd)?;
    let otel_mw = opentelemetry_surf::OpenTelemetryTracingMiddleware::default();
    let client = surf::client().with(otel_mw);
    let res = client.get("https://httpbin.org/get").await?;
    dbg!(res);

    opentelemetry::global::force_flush_tracer_provider();
    opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}
