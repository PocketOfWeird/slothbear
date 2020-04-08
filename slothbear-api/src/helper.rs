use std::env;

// -- Helper Functions -- //
pub fn get_app_base() -> String {
    env::var("ROCKET_APP_BASE").unwrap_or_default()
}
pub fn get_cas_base() -> String {
    env::var("ROCKET_CAS_BASE").unwrap_or_default()
}