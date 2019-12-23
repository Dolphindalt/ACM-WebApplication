use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};
use crypto::sha2::Sha256;
use jwt::{Header, Registered, Token};
use std::env;
use crate::models::user::User;

#[derive(Debug)]
pub struct APIKey(pub String);

pub fn read_token(key: &str) -> Result<String, String> {
    let secret = env::var("SECRET_KEY").expect("A secret key must be set");
    let token = Token::<Header, Registered>::parse(key)
        .map_err(|_| "Unable to parse key".to_string())?;
    if token.verify(secret.as_bytes(), Sha256::new()) {
        token.claims.sub.ok_or("Claims not valid".to_string())
    } else {
        Err("Token not valid".to_string())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for APIKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<APIKey, ()> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();
        if keys.len() != 1 {
            return Outcome::Forward(());
        }
        match read_token(keys[0]) {
            Ok(claim) => Outcome::Success(APIKey(claim)),
            Err(_) => Outcome::Forward(())
        }
    }
}

pub fn get_user_from_token_string(user_string: String) -> Option<User> {
    match serde_json::from_str::<User>(user_string.as_str()) {
        Ok(opt_user) => Some(opt_user),
        Err(_) => None,
    }
}