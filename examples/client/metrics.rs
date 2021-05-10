use opentelemetry::sdk::trace::Tracer;
use opentelemetry_surf::OpenTelemetryTracingMiddleware;

const SVC_NAME: &str = env!("CARGO_CRATE_NAME");

mod shared;

#[async_std::main]
async fn main() -> std::result::Result<(), http_types::Error> {
    femme::with_level(femme::LevelFilter::Info);
    shared::init_global_propagator();

    let tracer = pipeline();
    let otel_mw = OpenTelemetryTracingMiddleware::new(tracer);
    let client = create_client().with(otel_mw);

    // let uri = "https://httpbin.org/get";
    // let uri = "https://httpbin.org/image/svg";
    // let uri = "https://httpbin.org/drip?duration=3&numbytes=5&code=200&delay=1";
    // let uri = "https://httpbin.org/image/jpeg";
    let uri = "https://effigis.com/wp-content/uploads/2015/02/DigitalGlobe_WorldView2_50cm_8bit_Pansharpened_RGB_DRA_Rome_Italy_2009DEC10_8bits_sub_r_1.jpg";
    let res = client.get(uri).await?;
    dbg!(res);

    opentelemetry::global::force_flush_tracer_provider();
    opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}

fn pipeline() -> Tracer {
    opentelemetry_jaeger::new_pipeline()
        .with_service_name(SVC_NAME)
        .install_batch(opentelemetry::runtime::AsyncStd)
        .expect("pipeline install failure")
}

// more custom http client setup: use isahc with metrics enabled
fn create_client() -> surf::Client {
    use http_client::isahc::IsahcClient;
    use isahc::prelude::*;

    let isahc = HttpClient::builder()
        .default_headers(&[("user-agent", "surf/isahc (with request metrics)")])
        .metrics(true)
        .build()
        .expect("isahc client could no be created");
    let http_client = IsahcClient::from_client(isahc);
    surf::Client::with_http_client(http_client)
}
