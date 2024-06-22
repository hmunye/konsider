#[macro_use]
extern crate rocket;

use konsider_database::models::Post;
use konsider_database::{establish_connection, fetch_posts, insert_post};
use rocket::http::Method;
use rocket::serde::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct PostRequest {
    title: String,
    content: String,
}

#[derive(Serialize)]
struct ResponseData {
    data: Vec<Post>,
}

#[derive(Serialize, Deserialize)]
struct TestResponse {
    message: String,
}

#[get("/")]
fn index() -> &'static str {
    "Index: Hello, world!"
}

#[get("/data")]
fn json_test() -> Json<TestResponse> {
    let data: TestResponse = TestResponse {
        message: "Response From Rocket API".to_string(),
    };
    Json(data)
}

#[post("/create-post", data = "<data>")]
fn create_post(data: Json<PostRequest>) -> &'static str {
    let connection = &mut establish_connection();

    insert_post(connection, &data.title, &data.content);

    "success"
}

#[get("/posts")]
fn get_posts() -> Json<ResponseData> {
    let connection = &mut establish_connection();

    let results = fetch_posts(connection);

    Json(ResponseData { data: results })
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
    .to_cors()
    .expect("Failed to create CORS middleware");

    rocket::build()
        .attach(cors)
        .mount("/", routes![index])
        .mount("/", routes![json_test])
        .mount("/", routes![create_post])
        .mount("/", routes![get_posts])
}
