use actix_web::{web, App, Error, HttpServer};
use actix_web_opentelemetry::RequestTracing;
use services::Services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_batteries::init("sub");

    HttpServer::new(|| {
        App::new()
            .wrap(RequestTracing::new())
            .service(web::resource("/sub").to(sub))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await?;

    Ok(())
}

async fn sub(values: web::Json<Vec<i64>>) -> Result<web::Json<i64>, Error> {
    let mut total = values[0];
    let services = Services::new();

    for v in values.iter().skip(1) {
        total = services.add(total, -v).await?;
    }

    Ok(web::Json(total))
}
