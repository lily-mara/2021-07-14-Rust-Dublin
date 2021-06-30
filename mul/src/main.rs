use actix_web::{web, App, Error, HttpServer};
use actix_web_opentelemetry::RequestTracing;
use services::Services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_batteries::init("mul");

    HttpServer::new(|| {
        App::new()
            .wrap(RequestTracing::new())
            .service(web::resource("/mul").to(mul))
    })
    .bind("127.0.0.1:7070")?
    .run()
    .await?;

    Ok(())
}

async fn mul(values: web::Json<Vec<i64>>) -> Result<web::Json<i64>, Error> {
    let x = values[0];
    let y = values[1].abs();

    let services = Services::new();

    let mut total = x;
    for _ in 0..y {
        total = services.add(total, x).await?;
    }

    Ok(web::Json(total))
}
