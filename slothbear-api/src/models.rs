use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, Request, FromRequest};
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use uuid::Uuid;
use std::io::{Error, ErrorKind};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::helper;

// Derive JsonSchema for and request/response models
#[derive(Serialize, Deserialize, JsonSchema)]
// Set camelCase for derived Json
#[serde(rename_all = "camelCase")]
pub struct NewRender {
    pub path_scene: String,
    pub path_output: String,
    pub path_project: String,
    pub output_file_name: String,
    pub camera: String,
    pub frames: String,
    pub frame_width: u16,
    pub frame_height: u16,
}

// Derive JsonSchema for and request/response models
#[derive(Serialize, Deserialize, JsonSchema)]
// Set camelCase for derived Json
#[serde(rename_all = "camelCase")]
pub struct Render {
    pub id: String,
    #[serde(default = "default_renderer")]
    pub renderer: String,
    pub path_scene: String,
    pub path_output: String,
    pub path_project: String,
    pub output_file_name: String,
    pub camera: String,
    pub frames: String,
    #[serde(default = "default_frame_step")]
    pub frame_step: u16,
    pub frame_width: u16,
    pub frame_height: u16,
    #[serde(default = "default_split_chunks")]
    pub split_chunks: u16,
    pub user: String,
    pub time_submitted: u64,
}
// These functions will be called by serde if
// the specified input is not included in a Render
fn default_renderer() -> String {
    "Arnold".to_string()
}
fn default_frame_step() -> u16 {
    1
}
fn default_split_chunks() -> u16 {
    5
}
impl Render {
    pub fn from_json(json: Json<NewRender>, key: ApiKey) -> Render {
        let new_id = Uuid::new_v4().to_string();
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        return Render {
            id: new_id,
            renderer: default_renderer().to_owned(),
            path_scene: json.path_scene.to_owned(),
            path_output: json.path_output.to_owned(),
            path_project: json.path_project.to_owned(),
            output_file_name: json.output_file_name.to_owned(),
            camera: json.camera.to_owned(),
            frames: json.frames.to_owned(),
            frame_step: default_frame_step(),
            frame_width: json.frame_width.to_owned(),
            frame_height: json.frame_height.to_owned(),
            split_chunks: default_split_chunks(),
            user: key.user.id.to_owned(),
            time_submitted: time,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "serviceResponse")]
pub struct CasServiceResponse {
    #[serde(rename = "authenticationSuccess")]
    pub authentication_success: CasAuthenticationSuccess,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "authenticationSuccess")]
pub struct CasAuthenticationSuccess {
    pub user: String,
    pub attributes: CasAttributesMSU,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "attributes")]
pub struct CasAttributesMSU {
    pub first_name: String,
    pub last_name: String,
    pub name: String,
    pub email: String,
    #[serde(rename = "bearpass_Login")]
    pub bearpass_login: String,
    #[serde(rename = "bearpass_EmailID")]
    pub bearpass_email_id: String,
    pub primary_role: String,
    pub campus: String,
    #[serde(default, rename = "isFaculty")]
    pub is_faculty: Option<String>,
    #[serde(default, rename = "isStudent")]
    pub is_student: Option<String>,
    #[serde(default, rename = "isStaff")]
    pub is_staff: Option<String>,
}

// Derive JsonSchema for and request/response models
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub fname: String,
    pub lname: String,
    pub email: String,
    pub last_logon: u64,
    pub exp: u64,
}
impl User {
    pub fn from_cas_attributes_msu(cas: CasAttributesMSU) -> User {
        // set current unix epoch time as seconds
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        // set expiration as current time plus 12 hours
        let expiration_time = current_time + 43200;
        return User {
            id: cas.bearpass_login,
            fname: cas.first_name,
            lname: cas.last_name,
            email: cas.email,
            last_logon: current_time,
            exp: expiration_time
        };
    }
}

pub struct ApiKey {
    pub user: User
}
impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = std::io::Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, std::io::Error> {
        match request.headers().get_one("X-API-Key") {
            Some(key) => match helper::verify_api_key(key){
                Ok(authenticated_user) => Outcome::Success(ApiKey { user: authenticated_user }),
                Err(_e) => Outcome::Failure((Status::Unauthorized, Error::from(ErrorKind::InvalidInput)))
            },
            None => Outcome::Failure((Status::BadRequest, Error::from(ErrorKind::InvalidInput)))
        }
    }
}