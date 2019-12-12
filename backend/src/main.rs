#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate rocket_failure;
extern crate chrono;
extern crate dotenv;
extern crate crypto;
extern crate jwt;
extern crate rustc_serialize;

mod endpoints;
mod models;
mod schema;
mod db;

fn main() {
    let mut rocket = rocket::ignite().manage(db::connect());
    rocket = endpoints::user::mount(rocket);
    rocket = endpoints::password::mount(rocket);
    rocket = endpoints::usertype::mount(rocket);
    rocket.launch();
}
