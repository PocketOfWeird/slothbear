extern crate web_view;
extern crate serde;
extern crate blake3;
#[macro_use]
extern crate serde_json;

mod secret;

use web_view::*;
use serde::{Serialize, Deserialize, de};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewRender {
    pub path_scene: String,
    pub path_output: String,
    pub path_project: String,
    pub output_file_name: String,
    pub camera: String,
    pub frames: String,
    #[serde(deserialize_with = "from_str")]
    pub frame_width: u16,
    #[serde(deserialize_with = "from_str")]
    pub frame_height: u16,
    pub rp_user: Option<String>,
}
// A custom data serializer to explicitly convert the string values
// from the javascript form into another type
fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error> 
where  
    T: FromStr,
    T::Err: Display,
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    return T::from_str(&s).map_err(de::Error::custom);
}

fn submit_new_render(json: &str) {
    let new_render: NewRender = serde_json::from_str(json).unwrap();
    let header = json!({
        "Authorization": format!("Bearer {:?}", blake3::hash(secret::get_token_secret()))
    });
    println!("{:?}", header);
}

fn main() {
    web_view::builder()
        .title("Slothbear Submitter")
        .content(Content::Html(include_str!("static/index.html")))
        .size(800, 900)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| { 
            submit_new_render(&arg);
            Ok(())
        })
        .run()
        .unwrap();
}