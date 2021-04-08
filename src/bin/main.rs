extern crate diesel;
extern crate web_server;

use self::diesel::prelude::*;
use self::models::*;
use self::web_server::*;
use serde::{Serialize, Deserialize};
use actix_web::{error, post, get, web, App, Error, HttpResponse, HttpServer, Responder};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use futures::StreamExt;
use dotenv::dotenv;
use std::env;

const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[derive(Serialize, Deserialize)]
struct Claims {
    // aud: String,         // Optional. Audience
    exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    // iat: usize,          // Optional. Issued at (as UTC timestamp)
    // iss: String,         // Optional. Issuer
    // nbf: usize,          // Optional. Not Before (as UTC timestamp)
    sub: String,         // Optional. Subject (whom token refers to)
}

#[derive(Serialize, Deserialize)]
struct LoginReq {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct LoginRes {
    pub token: String,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello World Again!")
}

#[get("/hello")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/api/plan")]
async fn plan() -> impl Responder {
    use web_server::schema::plans::dsl::*;

    let connection = establish_connection();
    let results = plans
        .load::<Plan>(&connection)
        .expect("Error loading plans");

    let xs = serde_json::to_string(&results).unwrap();

    HttpResponse::Ok().body(xs)
}

#[post("/api/login")]
async fn login(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    dotenv().ok();

    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let req = serde_json::from_slice::<LoginReq>(&body)?;

    let claims = Claims { sub: req.email.to_owned(), exp: 10000000000 };

    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref())).expect("Failed to encode claims");

    let res = LoginRes { token: token };
    let xs = serde_json::to_string(&res).unwrap();
    Ok(HttpResponse::Ok().body(xs))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .service(index3)
            .service(plan)
            .service(login)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
