#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rs_es;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[cfg(test)]
mod tests;

mod documents;

use std::io;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::collections::HashSet;

use documents::get_cuesheets;
use documents::CuesheetDocument;

use rocket::response::{self, Response, Responder, content, NamedFile};
use rocket::http::Method;

struct CORS<R> {
    responder: R,
    allow_origin: &'static str,
    expose_headers: HashSet<&'static str>,
    allow_credentials: bool,
    allow_headers: HashSet<&'static str>,
    allow_methods: HashSet<Method>,
    max_age: Option<usize>
}

type PreflightCORS = CORS<()>;

impl PreflightCORS {
    pub fn preflight(origin: &'static str) -> PreflightCORS {
        CORS::origin((), origin)
    }
}

impl<'r, R: Responder<'r>> CORS<R> {
    pub fn origin(responder: R, origin: &'static str) -> CORS<R> {
        CORS {
            responder: responder,
            allow_origin: origin,
            expose_headers: HashSet::new(),
            allow_credentials: false,
            allow_headers: HashSet::new(),
            allow_methods: HashSet::new(),
            max_age: None
        }
    }

    pub fn any(responder: R) -> CORS<R> {
        CORS::origin(responder, "*")
    }

    pub fn credentials(mut self, value: bool) -> CORS<R> {
        self.allow_credentials = value;
        self
    }

    pub fn methods(mut self, methods: Vec<Method>) -> CORS<R> {
        for method in methods {
            self.allow_methods.insert(method);
        }

        self
    }

    pub fn headers(mut self, headers: Vec<&'static str>) -> CORS<R> {
        for header in headers {
            self.allow_headers.insert(header);
        }

        self
    }

}

impl<'r, R: Responder<'r>> Responder<'r> for CORS<R> {
    fn respond(self) -> response::Result<'r> {
        let mut response = Response::build_from(self.responder.respond()?)
            .raw_header("Access-Control-Allow-Origin", self.allow_origin)
            .finalize();

        match self.allow_credentials {
            true => response.set_raw_header("Access-Control-Allow-Credentials", "true"),
            false => response.set_raw_header("Access-Control-Allow-Credentials", "false")
        };

        if !self.allow_methods.is_empty() {
            let mut methods = String::with_capacity(self.allow_methods.len() * 7);
            for (i, method) in self.allow_methods.iter().enumerate() {
                if i != 0 { methods.push_str(", ") }
                methods.push_str(method.as_str());
            }

            response.set_raw_header("Access-Control-Allow-Methods", methods);
        }

        if !self.allow_headers.is_empty() {
            let mut headers = String::with_capacity(self.allow_headers.len() * 15);
            for (i, header) in self.allow_headers.iter().enumerate() {
                if i != 0 { headers.push_str(", ") }
                headers.push_str(header);
            }

            response.set_raw_header("Access-Control-Allow-Headers", headers);
        }


        Ok(response)
    }

}

#[route(OPTIONS, "/cuesheets/all")]
fn cors_preflight() -> PreflightCORS {
    CORS::preflight("http://localhost:8087")
        .methods(vec![Method::Options, Method::Post])
        .headers(vec!["Content-Type"])
}

#[get("/cuesheets/all")]
fn all_cuesheets() -> content::JSON<String> {
    match _get_cuesheets() {
        Err(e) => {
            println!("An error occured reading the cuesheet list: {:?}", e);
            return content::JSON("[]".to_string());
        },
        Ok(contents) => {
            return content::JSON(serde_json::to_string(&contents).unwrap());
        }
    };

}

fn _get_cuesheets() -> io::Result<Vec<CuesheetDocument>> {
    let res = match get_cuesheets() {
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
    rocket::ignite().mount("/", routes![index, static_files, all_cuesheets, cors_preflight])
}

fn main() {
    rocket().launch();
}