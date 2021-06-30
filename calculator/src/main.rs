use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

use actix_web::{web, App, Error, HttpServer, ResponseError};
use actix_web_opentelemetry::RequestTracing;

#[derive(Debug)]
struct RequestError(String);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_batteries::init("calculator");

    HttpServer::new(|| {
        App::new()
            .wrap(RequestTracing::new())
            .service(web::resource("/calculate").to(calculate))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}

async fn calculate(equation: web::Json<String>) -> Result<web::Json<i64>, Error> {
    let result = evaluate(&equation).await?;

    Ok(web::Json(result))
}

#[derive(Debug)]
enum RpnError {
    InvalidNumber,
    PopFromEmptyStack,
    NetworkError(Error),
}

impl From<Error> for RpnError {
    fn from(err: Error) -> Self {
        Self::NetworkError(err)
    }
}

impl ResponseError for RpnError {}

impl Display for RpnError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            RpnError::InvalidNumber => write!(f, "Not a valid number or operator"),
            RpnError::PopFromEmptyStack => write!(f, "Tried to operate on empty stack"),
            RpnError::NetworkError(e) => write!(f, "Network error: {}", e),
        }
    }
}

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
    let services = services::Services::new();

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
