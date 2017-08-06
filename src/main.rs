#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate rs_es;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate unescape;
extern crate comrak;

#[cfg(test)]
mod tests;

mod documents;

use std::io;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use rocket::response::{content, NamedFile};

#[get("/cuesheets/<id>")]
fn cuesheet_by_id(id: String) -> Option<content::Html<String>> {
    return match documents::get_cuesheet(&id) {
        Ok(cuesheet) => {
            let html = content::Html(*cuesheet);
            Some(html)
        },
        _ => None
    };
}

#[get("/search/phase/<phase>")]
fn search_by_phase(phase: String) -> rocket_contrib::Json<Vec<documents::CuesheetMetaData>> {
    match _query_cuesheets_by_phrase(&phase) {
        Err(e) => {
            println!("An error occured reading the cuesheet list: {:?}", e);
            let vec: Vec<documents::CuesheetMetaData> = Vec::new();
            return rocket_contrib::Json(vec);
        },
        Ok(contents) => {
            return rocket_contrib::Json(contents);
        }
    }
}

#[get("/search/<query>")]
fn search_cuesheets(query: String) -> content::Json<String> {

    match _query_cuesheets(&query) {
        Err(e) => {
            println!("An error occured reading the cuesheet list: {:?}", e);
            return content::Json("[]".to_string());
        },
        Ok(contents) => {
            return content::Json(serde_json::to_string(&contents).unwrap());
        }
    };
}

fn _query_cuesheets_by_phrase(phase: &str) -> io::Result<Vec<documents::CuesheetMetaData>> {
    let res = match documents::get_cuesheets_by_phase(phase) {
        Ok(cuesheets) => Ok(cuesheets),

        Err(_) => Err(Error::new(ErrorKind::InvalidData, "Getting search results failed"))
    };

    return res;
}

fn _query_cuesheets(query: &str) -> io::Result<Vec<documents::CuesheetMetaData>> {
    let res = match documents::get_cuesheets(query) {
        Ok(cuesheets) => Ok(cuesheets),

        Err(_) => Err(Error::new(ErrorKind::InvalidData, "Getting search results failed"))
    };

    return res;
}

#[get("/favicon.ico")]
fn favicon() -> io::Result<NamedFile> {
    NamedFile::open("public/favicon.ico")
}

#[get("/service-worker.js")]
fn service_worker() -> io::Result<NamedFile> {
    NamedFile::open("public/service-worker.js")
}

#[get("/index.html?<query>")]
fn index_precache(query: &str) -> io::Result<NamedFile> {
    NamedFile::open("public/index.html")
}


#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("public/index.html")
}

#[get("/static/<file..>")]

fn static_files(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new("public").join(file);
    //let filepath = path.as_path().as_os_str().to_os_string().into_string().unwrap();
    //println!("Path: {}", filepath);
    NamedFile::open(path).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, static_files, search_cuesheets,
        cuesheet_by_id, search_by_phase, favicon, service_worker, index_precache])
}

fn main() {
    rocket().launch();
}
