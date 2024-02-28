use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

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
    int1: i32,
    int2: i32,
    limit: i32,
    str1: String,
    str2: String,
}

#[get("/fizzbuzz")]
async fn get_fizzbuzz(query: web::Query<FizzBuzzQuery>) -> Result<impl Responder> {
    let response = FizzBuzzReponse {
        int1: query.int1,
        int2: query.int2,
        limit: query.limit,
        str1: query.str1.to_string(),
        str2: query.str2.to_string(),
    };
    Ok(web::Json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_fizzbuzz)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
