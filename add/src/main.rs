use actix_web::{web, App, HttpServer};
use actix_web_opentelemetry::RequestTracing;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_batteries::init("add");

    HttpServer::new(|| {
        App::new()
            .wrap(RequestTracing::new())
            .service(web::resource("/add").to(add))
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await?;

    Ok(())
}

async fn add(values: web::Json<Vec<i64>>) -> web::Json<i64> {
    web::Json(values.iter().sum())
}
