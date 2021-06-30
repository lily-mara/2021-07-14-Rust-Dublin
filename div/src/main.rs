use actix_web::{web, App, HttpServer};
use actix_web_opentelemetry::RequestTracing;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_batteries::init("div");

    HttpServer::new(|| {
        App::new()
            .wrap(RequestTracing::new())
            .service(web::resource("/div").to(div))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await?;

    Ok(())
}

async fn div(values: web::Json<Vec<i64>>) -> web::Json<i64> {
    let mut total = values[0];

    for v in values.iter().skip(1) {
        total /= *v;
    }

    web::Json(total)
}
