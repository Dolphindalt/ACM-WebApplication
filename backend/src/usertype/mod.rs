mod usertype;

use crate::db::Connection;
use rocket::{self};
use rocket_contrib::json::{Json, JsonValue};
use usertype::Usertype;
use rocket_failure::errors::*;

#[get("/<user_type_id>")]
fn read(user_type_id: i8, connection: Connection) -> ApiResult<Json<JsonValue>> {
    if let Some(user) = Usertype::read(user_type_id, &connection) {
        Ok(Json(json!(user)))
    } else {
        not_found!("User not found")
    }
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/usertype", routes![read])
}