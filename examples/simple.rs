#[async_std::main]
async fn main() -> surf::Result<()> {
    let (tracer, _uninstall) = opentelemetry_jaeger::new_pipeline().install().unwrap();
    let otel_mw = opentelemetry_surf::OpenTelemetryTracingMiddleware::new(tracer);
    let client = surf::client().with(otel_mw);
    let res = client.get("https://httpbin.org/get").await?;
    dbg!(res);
    Ok(())
}
