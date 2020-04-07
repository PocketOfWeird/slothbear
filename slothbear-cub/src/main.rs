extern crate hostname;
extern crate pickledb;
extern crate serde;

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod, error::Error};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct RenderJob {
    id: String,
}

fn load_db() -> Result<PickleDb, Error> {
    let filename = "slothbear-cub.db";
    if Path::new(filename).exists() {
        return PickleDb::load(filename, PickleDbDumpPolicy::AutoDump, SerializationMethod::Bin);
    } else {
        return Ok(PickleDb::new(filename, PickleDbDumpPolicy::AutoDump, SerializationMethod::Bin));
    }
}

fn main() {
    let mut db = load_db().expect("Unable to load the local database file");
    
    // Check Hostname
    let hname = db.get::<String>("hostname");
    if !hname.is_some() {
        let name = hostname::get().expect("Unable to retrieve computer hostname").into_string().expect("Unable to convert computer hostname");
        db.set("hostname", &name).unwrap();
    }
    println!("slothbear-cub: hostname is {}", db.get::<String>("hostname").unwrap());

    // Check config file if we are a Renderer or not

    // Check if we halted mid-job
    let leftover_job = db.get::<RenderJob>("job_current");
    if leftover_job.is_some() {
        // start the render job again
    }
    // Check if queue exists
    if db.lexists("jobs_todo") {
        // ?
    }
}
