use services::client::CalculationClient;
use std::collections::VecDeque;

use actix_web::{web, App, Error, ResponseError};

#[derive(Debug)]
struct RequestError(String);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    services::server::run(|| App::new().service(web::resource("/calculate").to(calculate))).await?;

    Ok(())
}

async fn calculate(equation: web::Json<String>) -> Result<web::Json<i64>, Error> {
    let result = evaluate(&equation).await?;

    Ok(web::Json(result))
}

#[derive(Debug, thiserror::Error)]
enum RpnError {
    #[error("Not a valid number or operator")]
    InvalidNumber,

    #[error("Tried to operate on an empty stack")]
    PopFromEmptyStack,

    #[error("Network error")]
    ClientError(#[from] services::client::Error),
}

impl ResponseError for RpnError {}

#[derive(Debug)]
struct RpnStack {
    stack: VecDeque<i64>,
}

impl RpnStack {
    fn new() -> RpnStack {
        RpnStack {
            stack: VecDeque::new(),
        }
    }

    fn push(&mut self, value: i64) {
        self.stack.push_front(value);
    }

    fn pop(&mut self) -> Result<i64, RpnError> {
        match self.stack.pop_front() {
            Some(value) => Ok(value),
            None => Err(RpnError::PopFromEmptyStack),
        }
    }
}

async fn evaluate(problem: &str) -> Result<i64, RpnError> {
    let mut stack = RpnStack::new();
    let services = CalculationClient::new();

    for term in problem.trim().split(' ') {
        match term {
            "+" => {
                let y = stack.pop()?;
                let x = stack.pop()?;
                stack.push(services.add(x, y).await?);
            }
            "-" => {
                let y = stack.pop()?;
                let x = stack.pop()?;
                stack.push(services.sub(x, y).await?);
            }
            "*" => {
                let y = stack.pop()?;
                let x = stack.pop()?;
                stack.push(services.mul(x, y).await?);
            }
            "/" => {
                let y = stack.pop()?;
                let x = stack.pop()?;
                stack.push(services.div(x, y).await?);
            }
            other => match other.parse() {
                Ok(value) => stack.push(value),
                Err(_) => return Err(RpnError::InvalidNumber),
            },
        }
    }

    let value = stack.pop()?;

    Ok(value)
}
