mod password;

use crate::db::Connection;
use rocket::{self};
use rocket_contrib::json::{Json, JsonValue};
use password::Password;
use crypto::sha2::Sha256;
use crate::crypto::digest::Digest;

#[post("/", data = "<password>")]
fn create(password: Json<Password>, connection: Connection) -> Json<Password> {
    let mut insert = Password { password_id: None, ..password.into_inner() };
    insert.password = seed_new_password(insert.password);
    Json(Password::create(insert, &connection))
}

#[put("/<password_id>", data = "<password>")]
fn update(password_id: i32, password: Json<Password>, connection: Connection) -> Json<JsonValue> {
    let mut update = Password { password_id: Some(password_id), ..password.into_inner() };
    update.password = seed_new_password(update.password);
    Json(json!({
        "success": Password::update(password_id, update, &connection)
    }))
}

#[delete("/<password_id>")]
fn delete(password_id: i32, connection: Connection) -> Json<JsonValue> {
    Json(json!({
        "success": Password::delete(password_id, &connection)
    }))
}

fn seed_new_password(password: String) -> String {
    let mut seed = Sha256::new();
    seed.input_str(password.as_str());
    seed.result_str()
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/password", routes![create, update, delete])
}