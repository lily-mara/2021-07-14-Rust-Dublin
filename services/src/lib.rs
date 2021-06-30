use actix_web::client::Client;
use actix_web::Error;
use actix_web_opentelemetry::ClientExt;

pub struct Services {
    client: Client,
}

impl Services {
    pub fn new() -> Self {
        Self {
            client: Client::default(),
        }
    }

    pub async fn add(&self, x: i64, y: i64) -> Result<i64, Error> {
        let result = self.send_request("http://add/add", [x, y]).await?;

        Ok(result)
    }

    pub async fn sub(&self, x: i64, y: i64) -> Result<i64, Error> {
        let result = self.send_request("http://sub/sub", [x, y]).await?;

        Ok(result)
    }

    pub async fn mul(&self, x: i64, y: i64) -> Result<i64, Error> {
        let result = self.send_request("http://mul/mul", [x, y]).await?;

        Ok(result)
    }

    pub async fn div(&self, x: i64, y: i64) -> Result<i64, Error> {
        let result = self.send_request("http://div/div", [x, y]).await?;

        Ok(result)
    }

    async fn send_request(&self, url: &str, values: [i64; 2]) -> Result<i64, Error> {
        let result = self
            .client
            .post(url)
            .trace_request()
            .send_json(&values)
            .await?
            .json()
            .await?;

        Ok(result)
    }
}
