mod user;

use crate::db::Connection;
use rocket::{self};
use rocket_contrib::json::{Json, JsonValue};
use user::User;

#[post("/", data = "<user>")]
fn create(user: Json<User>, connection: Connection) -> Json<User> {
    let insert = User { user_id: None, ..user.into_inner() };
    Json(User::create(insert, &connection))
}

#[get("/")]
fn read_all(connection: Connection) -> Json<JsonValue> {
    Json(json!(User::read_all(&connection)))
}

#[put("/<user_id>", data = "<user>")]
fn update(user_id: i32, user: Json<User>, connection: Connection) -> Json<JsonValue> {
    let update = User { user_id: Some(user_id), ..user.into_inner() };
    Json(json!({
        "success": User::update(user_id, update, &connection)
    }))
}

#[delete("/<user_id>")]
fn delete(user_id: i32, connection: Connection) -> Json<JsonValue> {
    Json(json!({
        "success": User::delete(user_id, &connection)
    }))
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/user", routes![create, read_all, update, delete])
}