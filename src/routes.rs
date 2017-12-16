use documents;
use playlists;
use serde_json;

use std::io;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use rocket::response::{content, NamedFile};

#[delete("/playlists/<id>/song/<song_id>")]
fn remove_song_from_playlist(id: String, song_id: String) -> content::Json<String> {
	return content::Json("{}".to_string());
}

#[put("/playlists/<id>/song/<song_id>")]
fn add_song_to_playlist(id: String, song_id: String) -> content::Json<String> {
	return content::Json("{}".to_string());
}

#[delete("/playlists/<id>")]
fn delete_playlist(id: String) -> content::Json<String> {
	return content::Json("{result: \"OK\"}".to_string());
}

#[put("/playlists")]
fn create_playlist() -> content::Json<String> {
	return content::Json("{}".to_string());
}

#[get("/playlists/<id>")]
fn playlist_by_id(id: String) -> content::Json<String> {
	return match playlists::playlist_by_id(&id) {
		Ok(playlist) => {
			content::Json(serde_json::to_string(&playlist).unwrap())
		},
		Err(e) => {
			println!("An error occured getting the playlist: {:?}", e);
			return content::Json("{}".to_string());
		}
	}
}

#[get("/playlists")]
fn get_playlists() -> content::Json<String> {
	return content::Json(serde_json::to_string(&playlists::get_playlists()).unwrap())
}

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