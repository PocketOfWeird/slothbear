use frank_jwt::{encode, decode, Algorithm, Error, ValidationOptions};
use serde_json;
use std::env;

use crate::models::User;

// -- Helper Functions -- //
pub fn get_app_base() -> String {
    env::var("ROCKET_APP_BASE").unwrap_or_default()
}
pub fn get_cas_base() -> String {
    env::var("ROCKET_CAS_BASE").unwrap_or_default()
}
pub fn generate_api_key(user: &User) -> Result<String, Error> {
    let header = json!({});
    let secret = env::var("ROCKET_KEY_SECRET").expect("Unable to find secret");
    let payload = serde_json::to_value(user)?;
    let jwt = encode(header, &secret, &payload, Algorithm::HS256)?;
    return Ok(jwt);
}
pub fn verify_api_key<'a>(key: &'a str) -> Result<User, Error> {
    let secret = env::var("ROCKET_KEY_SECRET").expect("Unable to find secret");
    let (_header, payload) = decode(key, &secret, Algorithm::HS256, &ValidationOptions::dangerous())?;
    let user: User = serde_json::from_value(payload)?;
    return Ok(user);
}