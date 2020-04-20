use rocket::response::Redirect;
use rocket::http::uri::Uri;
use rocket_contrib::json::Json;
use minreq;
use serde_xml_rs;

use crate::db;
use crate::helper;
use crate::models::{ApiKey, CasServiceResponse, NewRender, Render, User};

// GET: /api
#[openapi(skip)]
#[get("/")]
pub fn index(key: ApiKey) -> String {
    format!("Hello {}", key.user.fname)
}

// POST: /api/job
#[openapi]
#[post("/job", format = "json", data = "<new_render>")]
pub fn post_job(key: ApiKey, new_render: Json<NewRender>) -> Option<Json<Render>> {
    let render = Render::from_json(new_render, key);
    let submitted = db::send_job_to_queue(&render);
    if submitted {
        return Some(Json(render));
    } else {
        return None;
    }
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
pub fn auth_callback(ticket: String) -> Redirect {
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
        Redirect::to(
            format!(
                "{}/?authentication=failed&message={}",
                &helper::get_app_base(),
                Uri::percent_encode("Cas Authentication Failed")
            )
        )
    } else {
        // remove the "cas:" prepend from the xml string
        let filtered_response = response.replace("cas:", "");
        // convert the xml string to a CasResponse model
        let cas_response: CasServiceResponse = serde_xml_rs::from_str(&filtered_response).unwrap();
        // convert CasResponse model to User model
        let user = User::from_cas_attributes_msu(cas_response.authentication_success.attributes);
        // generate and api key and redirect to frontend
        match helper::generate_api_key(&user) {
            Ok(api_key) => return Redirect::to(
                format!(
                    "{}/?authentication=success&key={}",
                    &helper::get_app_base(),
                    api_key
                )
            ),
            Err(e) => return Redirect::to(
                format!(
                    "{}/?authentication=failed&message={}",
                    &helper::get_app_base(), 
                    Uri::percent_encode(&e.to_string())
                )
            )
        }
    }
}

// 404 Not Found Response
#[catch(404)]
pub fn not_found() -> Redirect {
    Redirect::to("/render/404.html")
}