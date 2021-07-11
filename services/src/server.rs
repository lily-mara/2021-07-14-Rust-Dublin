use actix_service::ServiceFactory;
use actix_web::{
    dev::{MessageBody, ServiceRequest, ServiceResponse},
    App, HttpServer,
};

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
    HttpServer::new(move || app_builder())
        .bind("0.0.0.0:80")?
        .run()
        .await?;

    Ok(())
}
