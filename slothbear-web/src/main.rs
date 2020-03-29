#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate rocket_okapi;
extern crate serde;
extern crate schemars;

use rocket_contrib::serve::StaticFiles;

#[cfg(test)] 
mod tests;

mod models;
mod routes;


fn main() {
    rocket::ignite()
    .mount( 
        "/slothbear", 
        routes_with_openapi![
                routes::index, 
                routes::post_render,
            ]
    ).mount("/", StaticFiles::from("public"))
    .launch();
}