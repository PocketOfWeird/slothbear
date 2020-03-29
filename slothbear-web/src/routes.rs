use rocket_contrib::json::Json;

use crate::models::{Render, RenderResponse};


// GET: /slothbear
#[openapi(skip)]
#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

// POST: /slothbear/render
#[openapi]
#[post("/render", data = "<_render>")]
pub fn post_render(_render: Json<Render>) -> Json<RenderResponse> {
    Json(RenderResponse {
        job_id: Some("abc123".to_owned()),
        status: "success".to_owned(),
    })
}
