use crate::db::Connection;
use rocket::{self};
use rocket::response::status::{Custom};
use rocket::http::RawStr;
use rocket::response::content::Html;
use rocket_contrib::json::{Json, JsonValue};
use rocket::http::Status;
use crate::models::password::Password;
use crate::models::user::User;
use regex::Regex;
use jwt::{
    Header,
    Registered,
    Token,
};
use crypto::sha2::Sha256;
use std::env;
use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

const MIN_PASSWORD_LEN: usize = 10;

#[derive(Serialize, Deserialize)]
struct NewUserMedium {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize)]
struct LoginMedium {
    pub email: String,
    pub password: String,
}

#[get("/register/<vercode>/<user_id>")]
fn registration_verification(vercode: &RawStr, user_id: i32, connection: Connection) -> Result<Html<&'static str>, Status> {
    if let Some(user) = User::get_by_id(user_id, &connection) {
        if let Some(password) = Password::get_by_password_id(user.password_id.unwrap(), &connection) {
            let mut update_password = password.clone();
            if password.verification_code.unwrap_or("invalid!".to_string()) == vercode.to_string() {
                update_password.verification_code = None;
                let did_update: bool = Password::update(user.password_id.unwrap(), update_password, &connection);
                if did_update {
                    return Ok(Html(r"
                        <html>
                            <body>
                                <p>You are now registered. You may close this page.</p> \
                            </body>
                        </html>
                    "));
                } else {
                    return Err(Status::InternalServerError);
                }
            }
            return Err(Status::NotFound);
        }
    }
    Err(Status::NotFound)
}

#[post("/register", data = "<new_user_medium>")]
fn create_user(new_user_medium: Json<NewUserMedium>, connection: Connection) -> Result<Json<User>, Custom<String>> {
    let medium = new_user_medium.into_inner();
    if !valid_email(&medium.email, &connection) {
        return Err(Custom(Status::BadRequest, "Please provide a valid email.".to_string()));
    }
    if !valid_password(&medium.password) {
        return Err(Custom(Status::BadRequest, format!("Password must be at least {} characters.", MIN_PASSWORD_LEN)));
    }
    let verification_code = generate_verification_code();
    let password = Password {
        password_id: None,
        password: medium.password,
        verification_code: Some(verification_code.clone()),
    };
    let inserted = Password::create(password, &connection);
    let user = User {
        user_id: None,
        password_id: inserted.password_id,
        user_type: 1,
        first_name: medium.first_name,
        last_name: medium.last_name,
        email: medium.email,
        points: 0.0,
    };
    let inserted = User::create(user, &connection);
    if !crate::email::send_verification_email(&inserted, verification_code) {
        User::delete(inserted.user_id.unwrap(), &connection);
        return Err(Custom(Status::InternalServerError, String::from("InternalServerError.")))
    }
    Ok(Json(inserted))
}

#[post("/login", data = "<login_medium>")]
fn attempt_login(login_medium: Json<LoginMedium>, connection: Connection) -> Result<Json<JsonValue>, Custom<String>> {
    let medium = login_medium.into_inner();
    if let Some(user) = User::get_user_by_email_and_password(&medium.email, &medium.password, &connection) {
        let header: Header = Default::default();
        let claims = Registered {
            sub: Some(serde_json::to_string(&user).ok().unwrap()),
            ..Default::default()
        };
        let token = Token::new(header, claims);
        let secret_key = env::var("SECRET_KEY").expect("Secret key must be set");

        token.signed(secret_key.as_bytes(), Sha256::new())
            .map(|message| Json(json!({ "token": message, "user": user })))
            .map_err(|_| Custom(Status::InternalServerError, String::from("InternalServerError")))
    } else {
        Err(Custom(Status::BadRequest, String::from("Invalid username or password")))
    }
}

fn valid_email(email: &String, connection: &Connection) -> bool {
    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
    if email_regex.is_match(&email) {
        return User::get_user_by_email(email, &connection).is_none();
    }
    false
}

fn valid_password(password: &String) -> bool {
    password.len() >= MIN_PASSWORD_LEN
}

fn generate_verification_code() -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(7)
        .collect()
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/auth", routes![create_user, attempt_login, registration_verification])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use super::LoginMedium;
    use super::NewUserMedium;
    use super::User;
    use crate::init_rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn test_registration_verification_good() {
        let mut rocket = init_rocket();
        rocket = rocket.mount("/auth", routes![super::registration_verification]);
        let client = Client::new(rocket).unwrap();
        let response = client.get("/auth/register/varcode/2").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_registration_verification_bad_user_id() {
        let mut rocket = init_rocket();
        rocket = rocket.mount("/auth", routes![super::registration_verification]);
        let client = Client::new(rocket).unwrap();
        let response = client.post("/auth/register/varcode/9119").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_attempt_login_bad_email_credential() {
        let mut rocket = init_rocket();
        rocket = rocket.mount("/auth", routes![super::attempt_login]);
        let client = Client::new(rocket).unwrap();
        let body_credentials: LoginMedium = LoginMedium {
            email: String::from("dcaron@mtech.edu"),
            password: String::from("mynamejeff"),
        };
        let body_json = serde_json::to_string::<LoginMedium>(&body_credentials).ok().unwrap();
        let response = client.post("/auth/login")
            .body(body_json)
            .dispatch();
        assert_eq!(response.status(), Status::BadRequest);
    }

    #[test]
    fn test_attempt_login_bad_password_credential() {
        let mut rocket = init_rocket();
        rocket = rocket.mount("/auth", routes![super::attempt_login]);
        let client = Client::new(rocket).unwrap();
        let body_credentials: LoginMedium = LoginMedium {
            email: String::from("jbraun@mtech.edu"),
            password: String::from("bad_password"),
        };
        let body_json = serde_json::to_string::<LoginMedium>(&body_credentials).ok().unwrap();
        let response = client.post("/auth/login")
            .body(body_json)
            .dispatch();
        assert_eq!(response.status(), Status::BadRequest);
    }

    #[test]
    fn test_attempt_login_good_credentials() {
        let mut rocket = init_rocket();
        rocket = rocket.mount("/auth", routes![super::attempt_login]);
        let client = Client::new(rocket).unwrap();
        let body_credentials: LoginMedium = LoginMedium {
            email: String::from("jbraun@mtech.edu"),
            password: String::from("mynamejeff"),
        };
        let response = client.post("/auth/login")
            .body(serde_json::to_string::<LoginMedium>(&body_credentials).ok().unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_create_user_good_values() {
        let mut rocket = init_rocket();
        rocket = rocket.mount("/auth", routes![super::create_user]);
        let client = Client::new(rocket).unwrap();
        let new_user_body: NewUserMedium  = NewUserMedium {
            email: String::from("pcurtiss@mtech.edu"),
            password: String::from("thehandofgod"),
            first_name: String::from("Phil"),
            last_name: String::from("Curtiss"),
        };
        let body_json = serde_json::to_string::<NewUserMedium>(&new_user_body).ok().unwrap();
        let mut response = client.post("/auth/register")
            .body(body_json)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.body_string().unwrap();
        let new_user_response: User = serde_json::from_str::<User>(&body).ok().unwrap();
        assert_eq!(new_user_body.email, new_user_response.email);
        assert_eq!(new_user_body.first_name, new_user_response.first_name);
        assert_eq!(new_user_body.last_name, new_user_response.last_name);
    }
}