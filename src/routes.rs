use documents;
use playlists;
use cuecards;
use serde_json;
use uuidcrate::Uuid;
use cuer_database::models::{Playlist, NewPlaylist, Cuecard};

use std::io;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use rocket_contrib::Json;
use rocket::response::{content, NamedFile};
use rocket::http::Status;

use diesel::QueryResult;
use guards::DbConn;

#[derive(Deserialize)]
struct FormPlaylist {
	name: String
}

#[derive(Serialize, Deserialize)]
struct FullPlaylist {
	id: i32,
	uuid: String,
	name: String,
	cuecards: Vec<Cuecard>
}

#[delete("/v2/playlists/<id>/cuesheet/<cuesheet_id>")]
fn remove_cuesheet_from_playlist(id: i32, cuesheet_id: i32, conn: DbConn) -> QueryResult<Json<usize>> {
	playlists::remove_cuesheet_from_playlist(&id, &cuesheet_id, &conn).map(|i| Json(i))
}

#[put("/v2/playlists/<id>/cuesheet/<cuesheet_id>")]
fn add_cuesheet_to_playlist(id: i32, cuesheet_id: i32, conn: DbConn) -> Result<Json<usize>, Status> {
	match playlists::add_cuesheet_to_playlist(&id, &cuesheet_id, &conn) {
		Ok(i) => Ok(Json(i)),
		_ => Err(Status::BadRequest)
	}
}

#[delete("/v2/playlists/<id>")]
fn delete_playlist(id: i32, conn: DbConn) -> QueryResult<Json<usize>> {
	playlists::delete_playlist(&id, &conn).map(|i| Json(i))
}

#[put("/v2/playlists", format="application/json", data="<playlist>")]
fn create_playlist(playlist: Json<FormPlaylist>, conn: DbConn) -> QueryResult<Json<Playlist>> {
	let data = playlist.into_inner();
	let u = Uuid::new_v4().hyphenated().to_string();

	let p = NewPlaylist {
		uuid: &u,
		name: &data.name
	};

	return playlists::create_playlist(&p, &conn).map(|p|Json(p));
}

#[get("/v2/playlists/<id>")]
fn playlist_by_id(id: i32, conn: DbConn) -> QueryResult<Json<Playlist>> {
	playlists::playlist_by_id(&id, &conn).map(|playlist| Json(playlist))
}

#[get("/v2/playlists")]
fn get_playlists(conn: DbConn) -> Json<Vec<FullPlaylist>> {
	let mut lists : Vec<FullPlaylist> = vec![];
	for p in  playlists::get_playlists(&conn).unwrap().into_iter() {
		let cuecards = playlists::get_cuecards(&p, &conn).unwrap();

		lists.push(FullPlaylist {
			id: p.id,
			uuid: p.uuid,
			name: p.name,
			cuecards
		});
	}

	return Json(lists);
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

#[get("/v2/search/<query>")]
fn search_cuecards(query: String, conn: DbConn) -> QueryResult<Json<Vec<Cuecard>>> {
	cuecards::search_cuecards(&query, &conn).map(|v| Json(v))
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