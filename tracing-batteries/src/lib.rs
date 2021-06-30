use opentelemetry::sdk::trace::Config;
use opentelemetry::sdk::trace::Sampler;
use opentelemetry::{global, sdk::propagation::TraceContextPropagator};
use tracing_subscriber::prelude::*;

pub fn init(service_name: &str) {
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name(service_name)
        .with_trace_config(Config::default().with_sampler(Sampler::AlwaysOn))
        .install_simple()
        .unwrap();

    global::set_text_map_propagator(TraceContextPropagator::new());

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = tracing_subscriber::Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
