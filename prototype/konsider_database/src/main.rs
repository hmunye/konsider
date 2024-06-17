// Author1: James Onley
// Date: May 28, 2024
// Description: This code uses the Rust Rocket framework to implement a basic API with several endpoints. 


#[macro_use] extern crate rocket;
use rocket::tokio::time::{ sleep, Duration} ;
use std::sync::atomic::AtomicUsize;
use rocket::State;
use std::sync::atomic::Ordering;
use konsider_database::*;

#[get("/")]
fn index() -> &'static str {
    "Index: Hello, world!"
}

#[get("/world")]
fn world() -> &'static str {
    let connection = &mut establish_connection();
    let title: &str = "Hello";
    let content: &str = "Hello World";
    create_post(connection, title, content);
    "success"
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
    rocket::build()
        .manage(HitCount { count: AtomicUsize::new(0) })
        .mount("/", routes![index])
        .mount("/", routes![world])
        .mount("/", routes![delay])
        .mount("/", routes![hello])
        .mount("/", routes![count])      
}