#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate rocket_failure;
extern crate rocket_cors;
extern crate chrono;
extern crate dotenv;
extern crate crypto;
extern crate jwt;
extern crate rustc_serialize;
extern crate regex;

mod endpoints;
mod models;
mod schema;
mod db;
mod auth;

use rocket_cors::{AllowedHeaders, AllowedOrigins, Error, Cors};
use rocket::http::Method;

pub fn init_rocket() -> rocket::Rocket {
    rocket::ignite().manage(db::connect())
}

fn init_cors() -> Result<Cors, Error> {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:4200"]);
    rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Post, Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }.to_cors()
}

fn main() {
    let cors = init_cors().ok().unwrap();
    let mut rocket = init_rocket();
    rocket = endpoints::login::mount(rocket);
    rocket = endpoints::event::mount(rocket);
    rocket.attach(cors)
        .launch();
}
