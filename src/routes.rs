use playlists;
use cuecards;
use programming;
use comrak::{markdown_to_html, ComrakOptions};
use uuidcrate::Uuid;
use cuer_database::models::{Playlist, PlaylistData, Cuecard};
use cuer_database::models::{Event, EventData}; //, Program, ProgramData, Tip, TipData, TipCuecard, TipCuecardData};

use std::io;
use std::path::{Path, PathBuf};

use rocket_contrib::json::Json;
use rocket::response::{content, NamedFile};
use rocket::http::Status;

use diesel::QueryResult;
use guards::DbConn;

#[derive(Deserialize)]
pub struct FormPlaylist {
	name: String
}

#[derive(Serialize, Deserialize)]
pub struct FullPlaylist {
	id: i32,
	uuid: String,
	name: String,
	cuecards: Vec<Cuecard>
}

#[derive(Serialize, Deserialize)]
pub struct FormEvent<'a> {
	name: String,
	date_start: String,
	date_end: String,
	schedule: Option<&'a str>,
	date_created: String,
	date_modified: String
}

#[delete("/v2/playlists/<uuid>/cuesheet/<cuesheet_uuid>")]
pub fn remove_cuesheet_from_playlist(uuid: String, cuesheet_uuid: String, conn: DbConn) -> QueryResult<Json<usize>> {
	playlists::remove_cuesheet_from_playlist(&uuid, &cuesheet_uuid, &conn).map(|i| Json(i))
}

#[put("/v2/playlists/<uuid>/cuesheet/<cuesheet_uuid>")]
pub fn add_cuesheet_to_playlist(uuid: String, cuesheet_uuid: String, conn: DbConn) -> Result<Json<String>, Status> {
	match playlists::add_cuesheet_to_playlist(&uuid, &cuesheet_uuid, &conn) {
		Ok(s) => Ok(Json(s)),
		_ => Err(Status::BadRequest)
	}
}

#[delete("/v2/playlists/<uuid>")]
pub fn delete_playlist(uuid: String, conn: DbConn) -> QueryResult<Json<Playlist>> {
	playlists::delete_playlist(&uuid, &conn).map(|p| Json(p))
}

#[put("/v2/playlists", format="application/json", data="<playlist>")]
pub fn create_playlist(playlist: Json<FormPlaylist>, conn: DbConn) -> QueryResult<Json<Playlist>> {
	let data = playlist.into_inner();
	let u = Uuid::new_v4().to_hyphenated().to_string();

	let p = PlaylistData {
		uuid: &u,
		name: &data.name
	};

	return playlists::create_playlist(&p, &conn).map(|p|Json(p));
}

/*#[get("/v2/playlists/<id>")]
fn playlist_by_id(id: i32, conn: DbConn) -> QueryResult<Json<Playlist>> {
	playlists::playlist_by_id(&id, &conn).map(|playlist| Json(playlist))
}*/

#[get("/v2/playlists")]
pub fn get_playlists(conn: DbConn) -> Json<Vec<FullPlaylist>> {
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

#[get("/v2/cuecards/<uuid>")]
pub fn cuecard_content_by_uuid(uuid: String, conn: DbConn) -> Result<content::Html<String>, Status> {
	match cuecards::get_cuesheet_content(&uuid, &conn) {
		Ok(cuecard) => {
			let markdown = markdown_to_html(&cuecard.content, &ComrakOptions::default());

			return Ok(content::Html(markdown));
		}
		_ => Err(Status::NotFound)
	}
}

#[get("/v2/search/<query>")]
pub fn search_cuecards(query: String, conn: DbConn) -> QueryResult<Json<Vec<Cuecard>>> {
	cuecards::search_cuecards(&query, &conn).map(|v| Json(v))
}


#[delete("/v2/events/<uuid>")]
pub fn delete_event(uuid: String, conn: DbConn) -> Result<Json<Event>, Status> {
	programming::delete_event(&uuid, &conn).map(|e| Json(e)).or_else(|_| Err(Status::NotFound))
}

#[get("/v2/events/<uuid>")]
pub fn event_by_uuid(uuid: String, conn: DbConn) -> Result<Json<Event>, Status> {
	programming::event_by_uuid(&uuid, &conn).map(|e| Json(e)).or_else(|_| Err(Status::NotFound))
}

#[get("/v2/events")]
pub fn get_events(conn: DbConn) -> Result<Json<Vec<Event>>, Status> {
	programming::get_events(&conn).map(|events| Json(events)).or_else(|_| Err(Status::BadRequest))
}

#[put("/v2/event", format="application/json", data="<event>")]
pub fn create_event(event: Json<FormEvent>, conn: DbConn) -> QueryResult<Json<Event>> {
	let data = event.into_inner();
	let u = Uuid::new_v4().to_hyphenated().to_string();

	let e = EventData {
		uuid: &u,
		name: &data.name,
		date_start: &data.date_start,
		date_end: &data.date_end,
		schedule: data.schedule,
		date_created: &data.date_created,
		date_modified: &data.date_modified
	};

	return programming::create_event(&e, &conn).map(|e|Json(e));
}

#[get("/favicon.ico")]
pub fn favicon() -> io::Result<NamedFile> {
	NamedFile::open("public/favicon.ico")
}

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
	NamedFile::open("public/index.html")
}

#[get("/static/<file..>")]
pub fn static_files(file: PathBuf) -> Option<NamedFile> {
	let path = Path::new("public").join(file);
	NamedFile::open(path).ok()
}