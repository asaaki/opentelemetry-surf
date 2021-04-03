use opentelemetry::global;
use opentelemetry_jaeger::Propagator as JaegerPropagator;

pub fn init_global_propagator() {
    global::set_text_map_propagator(JaegerPropagator::new());
}
