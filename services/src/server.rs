use actix_service::ServiceFactory;
use actix_web::{
    dev::{MessageBody, ServiceRequest, ServiceResponse},
    App, HttpServer,
};
use actix_web_opentelemetry::RequestTracing;
use opentelemetry::sdk::{
    propagation::TraceContextPropagator,
    trace::{Config, Sampler},
};
use tracing_subscriber::prelude::*;

pub async fn run<T, B>(app_builder: fn() -> App<T, B>) -> std::io::Result<()>
where
    B: 'static + MessageBody<Error = actix_http::Error>,
    T: 'static
        + ServiceFactory<
            ServiceRequest,
            Config = (),
            Response = ServiceResponse<B>,
            Error = actix_web::Error,
            InitError = (),
        >,
{
    init_tracing();

    HttpServer::new(move || app_builder().wrap(RequestTracing::new()))
        .bind("0.0.0.0:80")?
        .run()
        .await?;

    Ok(())
}

fn init_tracing() {
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_trace_config(Config::default().with_sampler(Sampler::AlwaysOn))
        .with_service_name(std::env::var("OTEL_EXPORTER_SERVICE_NAME").unwrap())
        .install_simple()
        .unwrap();

    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = tracing_subscriber::Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
