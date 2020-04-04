#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate rocket_okapi;
extern crate rand;
extern crate serde;
extern crate schemars;

#[cfg(test)] 
mod tests;

mod models;
mod routes;
mod service;


fn main() {
    rocket::ignite()
    .mount( 
        "/slothbear", 
        routes_with_openapi![
                routes::index, 
                routes::post_render,
            ]
    )
    .launch();
}