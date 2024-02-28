use actix_web::{error, get, web, App, HttpResponse, HttpServer, Responder, Result};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

use crate::adapters::driven::fizzbuzz;
use crate::ports::FizzBuzzCommand;
use crate::ports::FizzBuzzer;

mod adapters;
mod domains;
mod ports;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize)]
struct FizzBuzzQuery {
    int1: i32,
    int2: i32,
    limit: i32,
    str1: String,
    str2: String,
}

#[derive(Serialize)]
struct FizzBuzzReponse {
    response: Vec<String>,
}

#[derive(Debug, Display, Error)]
struct MyError {
    name: String,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for MyError {}

#[get("/fizzbuzz")]
async fn get_fizzbuzz(query: web::Query<FizzBuzzQuery>) -> Result<impl Responder, MyError> {
    let fizzbuzzer = fizzbuzz::Simple {};
    let command = FizzBuzzCommand {
        int1: query.int1,
        int2: query.int2,
        limit: query.limit,
        str1: query.str1.to_string(),
        str2: query.str2.to_string(),
    };
    match fizzbuzzer.fizzbuzz(command) {
        Ok(response) => Ok(web::Json(response)),
        Err(err) => Err(MyError { name: err }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(get_fizzbuzz))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
