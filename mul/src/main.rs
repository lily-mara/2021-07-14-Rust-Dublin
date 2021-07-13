use actix_web::{web, App, Error};
use services::client::CalculationClient;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    services::server::run(|| App::new().service(web::resource("/mul").to(mul))).await?;

    Ok(())
}

#[tracing::instrument]
async fn mul(values: web::Json<Vec<i64>>) -> Result<web::Json<i64>, Error> {
    let x = values[0];
    let y = values[1].abs();

    let client = CalculationClient::new();

    let mut total = x;
    for _ in 1..y {
        let y = total;
        total = client.add(total, x).await?;
        tracing::info!(x, y, total, "add result");
    }

    tracing::info!(x, y, total, "multiplication result");

    Ok(web::Json(total))
}
