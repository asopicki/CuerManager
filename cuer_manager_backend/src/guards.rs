use rocket::Request;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};

pub struct BackendConfig {
    pub music_files_dir: String,
    pub indexer_path: String,
    pub cuecards_lib_dir: String,
    pub db_url: String,
    pub cuecards_self_managed: bool,
    pub minutes_per_tip: u32,
}

#[derive(Debug)]
pub struct FileNameHeader(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for FileNameHeader {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let headers: Vec<&str> = request.headers().get("X-Input-FileName").collect();

        if headers.is_empty() {
            return Outcome::Failure((Status::BadRequest, ()));
        }
        
        Outcome::Success(FileNameHeader(String::from(headers[0])))
    }
}