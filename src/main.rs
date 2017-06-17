#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
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

use documents::{get_cuesheets, get_cuesheet};
use documents::CuesheetMetaData;

use rocket::response::{content, NamedFile};

#[get("/cuesheets/<id>")]
fn cuesheet_by_id(id: &str) -> Option<content::HTML<String>> {
    return match get_cuesheet(id) {
        Ok(cuesheet) => {
            let html = content::HTML(*cuesheet);
            Some(html)
        },
        _ => None
    };
}

#[get("/search/<query>")]
fn search_cuesheets(query: &str) -> content::JSON<String> {

    if query.eq("all") {
        match _query_cuesheets(query) {
            Err(e) => {
                println!("An error occured reading the cuesheet list: {:?}", e);
                return content::JSON("[]".to_string());
            },
            Ok(contents) => {
                return content::JSON(serde_json::to_string(&contents).unwrap());
            }
        };
    } else {
        return content::JSON("[]".to_string());
    }
}

fn _query_cuesheets(query: &str) -> io::Result<Vec<CuesheetMetaData>> {
    let res = match get_cuesheets(query) {
        Ok(cuesheets) => Ok(cuesheets),

        Err(_) => Err(Error::new(ErrorKind::InvalidData, "Getting search results failed"))
    };

    return res;
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/js/dist/index.html")
}

#[get("/static/<file..>")]

fn static_files(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new("static/js/dist/static").join(file);
    //let filepath = path.as_path().as_os_str().to_os_string().into_string().unwrap();
    //println!("Path: {}", filepath);
    NamedFile::open(path).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, static_files, search_cuesheets, cuesheet_by_id])
}

fn main() {
    rocket().launch();
}