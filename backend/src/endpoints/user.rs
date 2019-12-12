use crate::db::Connection;
use rocket::{self};
use rocket_contrib::json::{Json, JsonValue};
use crate::models::user::User;
use rocket_failure::errors::*;

#[post("/", data = "<user>")]
fn create(user: Json<User>, connection: Connection) -> ApiResult<Json<User>> {
    let insert = User { user_id: None, ..user.into_inner() };
    Ok(Json(User::create(insert, &connection)))
}

#[get("/")]
fn read_all(connection: Connection) -> ApiResult<Json<JsonValue>> {
    Ok(Json(json!(User::read_all(&connection))))
}

#[put("/<user_id>", data = "<user>")]
fn update(user_id: i32, user: Json<User>, connection: Connection) -> ApiResult<Json<JsonValue>> {
    let update = User { user_id: Some(user_id), ..user.into_inner() };
    Ok(Json(json!({
        "success": User::update(user_id, update, &connection)
    })))
}

#[delete("/<user_id>")]
fn delete(user_id: i32, connection: Connection) -> ApiResult<Json<JsonValue>> {
    Ok(Json(json!({
        "success": User::delete(user_id, &connection)
    })))
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/user", routes![create, read_all, update, delete])
}