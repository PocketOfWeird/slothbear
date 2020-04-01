use rocket_contrib::json::Json;

use crate::models::{Render, RenderResponse};
use crate::service::send_to_rp;

// GET: /slothbear
#[openapi(skip)]
#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

// POST: /slothbear/render
#[openapi]
#[post("/render", format = "json", data = "<render>")]
pub fn post_render(render: Json<Render>) -> Json<RenderResponse> {
    let new_render = Render::from_json(render);
    let response = send_to_rp(new_render);
    return Json(response);
}
