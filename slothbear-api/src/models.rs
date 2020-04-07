use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

// Derive JsonSchema for and request/response models
#[derive(Serialize, Deserialize, JsonSchema)]
// Set camelCase for derived Json
#[serde(rename_all = "camelCase")]
pub struct Render {
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
    pub rp_user: Option<String>,
    pub rp_job_name: Option<String>,
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
    pub fn from_json(json: Json<Render>) -> Render {
        return Render {
            renderer: json.renderer.to_owned(),
            path_scene: json.path_scene.to_owned(),
            path_output: json.path_output.to_owned(),
            path_project: json.path_project.to_owned(),
            output_file_name: json.output_file_name.to_owned(),
            camera: json.camera.to_owned(),
            frames: json.frames.to_owned(),
            frame_step: json.frame_step.to_owned(),
            frame_width: json.frame_width.to_owned(),
            frame_height: json.frame_height.to_owned(),
            split_chunks: json.split_chunks.to_owned(),
            rp_user: json.rp_user.to_owned(),
            rp_job_name: json.rp_job_name.to_owned(),
        }
    }
}


// Derive JsonSchema for and request/response models
#[derive(Serialize, Deserialize, JsonSchema)]
// Set camelCase for derived Json
#[serde(rename_all = "camelCase")]
pub struct RenderResponse {
    pub job_id: Option<String>,
    pub status: String,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
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
pub struct User {
    pub id: String,
    pub fname: String,
    pub lname: String,
    pub email: String,
}
impl User {
    pub fn from_cas_attributes_msu(cas: CasAttributesMSU) -> User {
        return User {
            id: cas.bearpass_login,
            fname: cas.first_name,
            lname: cas.last_name,
            email: cas.email
        };
    }
}
