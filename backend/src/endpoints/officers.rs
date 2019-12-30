use crate::db::Connection;
use rocket::{self};
use rocket_contrib::json::{Json, JsonValue};
use crate::models::user::User;
use crate::models::user::UserModel;
use rocket::response::status::{Custom};

#[get("/")]
fn get_all(connection: Connection) -> Result<Json<JsonValue>, Custom<String>> {
    let officers: Vec<UserModel> = User::into_user_models(&User::get_officers, &connection);
    Ok(Json(json!{officers}))
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/officers", routes![get_all])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use crate::init_rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn get_all_officers_test_good() {
        let mut rocket = init_rocket();
        rocket = rocket.mount("/event", routes![super::get_all]);
        let client = Client::new(rocket).unwrap();
        let request = client.get("/event").dispatch();
        assert_eq!(request.status(), Status::Ok);
    }
}