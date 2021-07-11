use actix_service::ServiceFactory;
use actix_web::{
    dev::{MessageBody, ServiceRequest, ServiceResponse},
    App, HttpServer,
};
use actix_web_opentelemetry::RequestTracing;
use opentelemetry::{
    global,
    sdk::{
        propagation::TraceContextPropagator,
        trace::{Config, Sampler},
    },
};
use tracing_subscriber::layer::SubscriberExt;

fn init_tracing(service_name: &str) {
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
    init_tracing(
        std::env::var("OTEL_EXPORTER_SERVICE_NAME")
            .unwrap()
            .as_ref(),
    );

    HttpServer::new(move || app_builder().wrap(RequestTracing::new()))
        .bind("0.0.0.0:80")?
        .run()
        .await?;

    Ok(())
}
