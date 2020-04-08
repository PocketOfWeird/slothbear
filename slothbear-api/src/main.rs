#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate rocket_okapi;
extern crate minreq;
extern crate rand;
extern crate serde;
extern crate serde_xml_rs;
extern crate schemars;

use rocket::config::Config;
use rocket_contrib::serve::StaticFiles;
use std::env;

#[cfg(test)] 
mod tests;

mod helper;
mod models;
mod routes;
mod service;


fn main() {
    let port: u16 = match env::var("HTTP_PLATFORM_PORT") {
        Ok(val) => val.parse().unwrap(),
        Err(_e) => 8000,
    };
    let static_dir: String = match env::var("ROCKET_STATIC_DIR") {
        Ok(val) => val.to_string(),
        Err(_e) => "static".to_string(),
    };

    let mut config = Config::active().unwrap();
    config.set_port(port);

    rocket::custom(config)
    .mount("/render", StaticFiles::from(static_dir))
    .mount("/render/auth", routes![
            routes::auth_login,
            routes::auth_logout,
            routes::auth_callback,
        ]
    )
    .mount( 
        "/render/api", 
        routes_with_openapi![
                routes::index, 
                routes::post_job,
            ]
    )
    .register(catchers![routes::not_found])
    .launch();
}