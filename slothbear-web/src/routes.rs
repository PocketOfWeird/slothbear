use rocket_contrib::json::Json;

use crate::models::{Render};


// GET: /slothbear
#[openapi(skip)]
#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

// POST: /slothbear/render
#[openapi]
#[post("/render", data = "<render>")]
pub fn post_render(render: Json<Render>) -> Json<Render> {
    render
}
