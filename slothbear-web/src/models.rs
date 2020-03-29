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
    pub frame_step: i32,
    pub frame_width: i32,
    pub frame_height: i32,
    #[serde(default = "default_split_chunks")]
    pub split_chunks: i32,
    pub rp_user: Option<String>,
    pub rp_job_name: Option<String>,
}
// These functions will be called by serde if
// the specified input is not included in a Render
fn default_renderer() -> String {
    "Arnold".to_string()
}
fn default_frame_step() -> i32 {
    1
}
fn default_split_chunks() -> i32 {
    5
}

// Derive JsonSchema for and request/response models
#[derive(Serialize, Deserialize, JsonSchema)]
// Set camelCase for derived Json
#[serde(rename_all = "camelCase")]
pub struct RenderResponse {
    pub job_id: Option<String>,
    pub status: String
}