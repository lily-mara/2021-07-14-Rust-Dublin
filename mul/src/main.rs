use actix_web::{web, App, Error};
use services::client::CalculationClient;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    services::server::run(|| App::new().service(web::resource("/mul").to(mul))).await?;

    Ok(())
}

async fn mul(values: web::Json<Vec<i64>>) -> Result<web::Json<i64>, Error> {
    let x = values[0];
    let y = values[1].abs();

    let client = CalculationClient::new();

    let mut total = x;
    for _ in 0..y {
        total = client.add(total, x).await?;
    }

    Ok(web::Json(total))
}
