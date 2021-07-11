use actix_web::{web, App, Error};
use services::client::CalculationClient;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    services::server::run(|| App::new().service(web::resource("/sub").to(sub))).await?;

    Ok(())
}

async fn sub(values: web::Json<Vec<i64>>) -> Result<web::Json<i64>, Error> {
    let mut total = values[0];
    let client = CalculationClient::new();

    for v in values.iter().skip(1) {
        total = client.add(total, -v).await?;
    }

    Ok(web::Json(total))
}
