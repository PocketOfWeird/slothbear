use rocket::response::Redirect;
use rocket::http::uri::Uri;
use rocket_contrib::json::Json;
use minreq;
use serde_xml_rs;

use crate::helper;
use crate::models::{CasServiceResponse, Render, RenderResponse, User};
use crate::service::send_to_rp;

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
            &helper::get_cas_base(), 
            Uri::percent_encode(
                &format!(
                    "{}/auth/callback",
                    &helper::get_app_base()
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
            &helper::get_cas_base(), 
            Uri::percent_encode(
                &helper::get_app_base()
            )
        )
    )
}

// GET: /auth/callback
#[get("/callback?<ticket>")]
pub fn auth_callback(ticket: String) -> String {
    // request the cas server to validate the ticket
    let response: String = minreq::get(
        format!("{}/serviceValidate?service={}&ticket={}", 
            &helper::get_cas_base(), 
            Uri::percent_encode(
                &format!(
                    "{}/auth/callback",
                    &helper::get_app_base()
                )
            ),
            &ticket
        ) // Convert the recieved xml response to a string
    ).send().unwrap().as_str().unwrap().to_owned();

    // check if the xml string has "cas:authenticationFailure"
    if response.contains("cas:authenticationFailure") {
        return "failure".to_owned();
        // route to /#casAuthenticateFailure 
    } else {
        // remove the "cas:" prepend from the xml string
        let filtered_response = response.replace("cas:", "");
        // convert the xml string to a CasResponse model
        let cas_response: CasServiceResponse = serde_xml_rs::from_str(&filtered_response).unwrap();
        let user = User::from_cas_attributes_msu(cas_response.authentication_success.attributes);
        return user.id.to_owned();
        // route to /#casAuthenticateSuccess
    }
}

// 404 Not Found Response
#[catch(404)]
pub fn not_found() -> Redirect {
    Redirect::to("/render/404.html")
}