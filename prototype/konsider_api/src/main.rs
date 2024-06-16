// Author: James Onley
// Date: May 28, 2024
// Description: This code uses the Rust Rocket framework to implement a basic API with several endpoints. 

#[macro_use] extern crate rocket;

use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicUsize;
use rocket::tokio::time::{sleep, Duration};
use rocket::State;
use rocket::serde::json::Json;
use rocket::http::Method;
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, CorsOptions,
};
use serde::{Serialize, Deserialize};

#[get("/")]
fn index() -> &'static str {
    "Index: Hello, world!"
}

#[derive(Serialize, Deserialize)]
struct TestResponse {
    message: String
}

#[get("/data")]
fn json_test() -> Json<TestResponse> {
    let data: TestResponse = TestResponse {
        message: "Response From Rocket API".to_string()
    };
    Json(data)
}

#[get("/world")]
fn world() -> &'static str {
    "World: Hello, world!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs( seconds )).await;
    format!("Delay: Waited for {} seconds", seconds)
}

#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("Hello: You're a cool {} year old, {}!", age, name)
    } else {
        format!("Hello: {}, we need to talk about your coolness.", name)
    }
}

struct HitCount{
    count: AtomicUsize
}

#[get("/count")]
fn count(hit_count:&State<HitCount>) -> String {
    let current_count = hit_count.count.load(Ordering::Relaxed);
    format!("Count: Number of visits: {}", current_count)
}

#[launch]
fn rocket() -> _ {
    let allowed_origins: rocket_cors::AllOrSome<rocket_cors::Origins> = AllowedOrigins::all();
    let cors: rocket_cors::Cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Options]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().expect("Failed to create CORS middleware");

    rocket::build()
        .attach(cors)
        .manage(HitCount { count: AtomicUsize::new(0) })
        .mount("/", routes![index])
        .mount("/", routes![json_test])
        .mount("/", routes![world])
        .mount("/", routes![delay])
        .mount("/", routes![hello])
        .mount("/", routes![count])      
}