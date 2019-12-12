#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
extern crate chrono;
extern crate dotenv;

mod password;
mod user;
mod usertype;
mod schema;
mod db;

fn main() {
    let mut rocket = rocket::ignite().manage(db::connect());
    rocket = user::mount(rocket);
    rocket.launch();
}
