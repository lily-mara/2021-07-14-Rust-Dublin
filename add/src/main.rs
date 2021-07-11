use actix_web::{web, App};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    services::server::run(|| App::new().service(web::resource("/add").to(add))).await?;

    Ok(())
}

async fn add(values: web::Json<Vec<i64>>) -> web::Json<i64> {
    web::Json(values.iter().sum())
}
