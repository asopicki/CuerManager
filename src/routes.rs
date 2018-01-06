use documents;
use playlists;
use serde_json;

use std::io;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use rocket_contrib::Json;
use rocket::response::{content, NamedFile};
use rocket::http::Status;

#[delete("/playlists/<id>/cuesheet/<cuesheet_id>")]
fn remove_cuesheet_from_playlist(id: String, cuesheet_id: String) -> Result<content::Json<String>, Status> {
	return match playlists::remove_cuesheet_from_playlist(&id, &cuesheet_id) {
		Ok(playlist) => Ok(content::Json(serde_json::to_string(&playlist).unwrap())),
		Err(e) => {
			println!("An error occured adding the cuesheet to playlist: {:?}", e);
			return Err(Status::BadRequest);
		}
	}
}

#[put("/playlists/<id>/cuesheet/<cuesheet_id>")]
fn add_cuesheet_to_playlist(id: String, cuesheet_id: String) -> Result<content::Json<String>, Status> {
	return match playlists::add_cuesheet_to_playlist(&id, &cuesheet_id) {
		Ok(playlist) => {
			Ok(content::Json(serde_json::to_string(&playlist).unwrap()))
		},
		Err(e) => {
			println!("An error occured adding the cuesheet to playlist: {:?}", e);
			return Err(Status::BadRequest);
		}
	}
}

#[delete("/playlists/<id>")]
fn delete_playlist(id: String) -> Result<content::Json<String>, Status> {
	return match playlists::delete_playlist(&id) {
		Ok(_) => Ok(content::Json("{result: \"OK\", id: \"" + id + "\"}".to_string())),
		Err(e) => {
			println!("An error occured adding the cuesheet to playlist: {:?}", e);
			return Err(Status::NotFound);
		}
	}
}

#[put("/playlists", format="application/json", data="<playlist>")]
fn create_playlist(playlist: Json<playlists::Playlist>) -> Result<content::Json<String>, Status> {
	match playlists::create_playlist(playlist.into_inner()) {
		Ok(playlist) => {
			Ok(content::Json(serde_json::to_string(&playlist).unwrap()))
		}
		Err(e) => {
			println!("An error occured creating the playlist: {:?}", e);
			return Err(Status::BadRequest);
		}
	}
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
	return match playlists::get_playlists() {
		Ok(playlists) => {
			content::Json(serde_json::to_string(&playlists).unwrap())
		},
		Err(e) => {
			println!("An error occured getting the playlist: {:?}", e);
			return content::Json("[]".to_string());
		}
	}

}

#[get("/cuesheets/<id>")]
fn cuesheet_by_id(id: String) -> Option<content::Html<String>> {
	return match documents::get_cuesheet_content(&id) {
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