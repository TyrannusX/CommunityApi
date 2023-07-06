// define compiler features
#![feature(proc_macro_hygiene, decl_macro)]

// define external crates being used
#[macro_use]
extern crate rocket;
extern crate rocket_cors;
extern crate chrono;

// internal modules
mod handlers;
mod domain;
mod traits;
mod services;

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

fn main() {
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete, Method::Options].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().expect("Failed to build cors");

    rocket::ignite()
        .mount("/", routes![
            handlers::communities::get_communities, 
            handlers::posts::get_posts])
        .attach(cors)
        .launch();
}
