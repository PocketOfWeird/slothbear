use rocket::response::Redirect;
use rocket::http::uri::Uri;
use rocket_contrib::json::Json;
use std::env;

use crate::models::{Render, RenderResponse};
use crate::service::send_to_rp;

// -- Helper Functions -- //
fn get_app_base() -> String {
    env::var("ROCKET_APP_BASE").unwrap_or_default()
}
fn get_cas_base() -> String {
    env::var("ROCKET_CAS_BASE").unwrap_or_default()
}

// GET: /api
#[openapi(skip)]
#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

// POST: /api/job
#[openapi]
#[post("/job", format = "json", data = "<render>")]
pub fn post_job(render: Json<Render>) -> Json<RenderResponse> {
    let new_render = Render::from_json(render);
    let response = send_to_rp(new_render);
    return Json(response);
}

// GET: /auth/login
#[get("/login")]
pub fn auth_login() -> Redirect {
    Redirect::to(
        format!(
            "{}/login?service={}", 
            &get_cas_base(), 
            Uri::percent_encode(
                &format!(
                    "{}/auth/callback",
                    &get_app_base()
                )
            )
        )
    )
}

// GET: /auth/logout
#[get("/logout")]
pub fn auth_logout() -> Redirect {
    Redirect::to(
        format!(
            "{}/logout?url={}", 
            &get_cas_base(), 
            Uri::percent_encode(
                &get_app_base()
            )
        )
    )
}

// GET: /auth/callback
#[get("/callback?<ticket>")]
pub fn auth_callback(ticket: String) -> Redirect {
    // validate ticket
    // redirect to homepage
    Redirect::to(format!("{}", &get_app_base()))
}