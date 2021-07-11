use actix_web::ResponseError;
use awc::Client;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error sending HTTP request")]
    SendRequestError(#[from] awc::error::SendRequestError),

    #[error("Error deserializing response body as JSON")]
    JsonPayloadError(#[from] awc::error::JsonPayloadError),
}

impl ResponseError for Error {}

pub struct CalculationClient {
    client: Client,
}

impl CalculationClient {
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
            .send_json(&values)
            .await?
            .json()
            .await?;

        Ok(result)
    }
}
