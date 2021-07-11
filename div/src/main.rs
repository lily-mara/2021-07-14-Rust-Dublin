use actix_web::{web, App};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    services::server::run(|| App::new().service(web::resource("/div").to(div))).await?;

    Ok(())
}

async fn div(values: web::Json<Vec<i64>>) -> web::Json<i64> {
    let mut total = values[0];

    for v in values.iter().skip(1) {
        total /= *v;
    }

    web::Json(total)
}
