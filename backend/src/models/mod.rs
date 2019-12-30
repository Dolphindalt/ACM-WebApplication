pub mod event_file;
pub mod event_type;
pub mod event;
pub mod file;
pub mod user;
pub mod user_profile;
pub mod password;
pub mod user_type;

use crypto::sha2::Sha256;
use crate::crypto::digest::Digest;

fn seed_new_password(password: String) -> String {
    let mut seed = Sha256::new();
    seed.input_str(password.as_str());
    seed.result_str()
}