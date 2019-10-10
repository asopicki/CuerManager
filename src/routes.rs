use comrak::{markdown_to_html, ComrakOptions};
use crate::cuecards;
use cuer_database;
use cuer_database::models::{Cuecard, Playlist, PlaylistData};
use cuer_database::models::{Event, EventData, Program, ProgramData, Tip, TipCuecardData, TipData}; //TipCuecard};
use crate::playlists;
use crate::programming;
use uuidcrate::Uuid;

use std::convert::From;
use std::io;
use std::path::{Path, PathBuf};

use rocket::http::Status;
use rocket::response::{content, NamedFile};
use rocket_contrib::json::Json;

use chrono::prelude::*;

use super::DbConn;
use diesel::QueryResult;

#[derive(Deserialize)]
pub struct FormPlaylist {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct FullPlaylist {
    id: i32,
    uuid: String,
    name: String,
    cuecards: Vec<Cuecard>,
}

#[derive(Serialize, Deserialize)]
pub struct FormEvent<'a> {
    name: String,
    date_start: String,
    date_end: String,
    schedule: Option<&'a str>,
    date_created: String,
    date_modified: String,
}

#[derive(Serialize, Deserialize)]
pub struct FullTip {
    //Tip including cue cards
    id: i32,
    uuid: String,
    name: String,
    program_id: i32,
    date_start: String,
    date_end: String,
    cuecards: Vec<Cuecard>,
}

impl From<Tip> for FullTip {
    fn from(tip: Tip) -> Self {
        FullTip {
            id: tip.id,
            uuid: tip.uuid,
            name: tip.name,
            program_id: tip.program_id,
            date_start: tip.date_start,
            date_end: tip.date_end,
            cuecards: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FormTip {
    name: String,
    program_id: i32,
    date_start: String,
    date_end: String,
}

#[derive(Serialize, Deserialize)]
pub struct FormTipCuecard {
    tip_uuid: String,
    cuecard_uuid: String,
    sort_order: i32,
}

#[delete("/v2/playlists/<uuid>/cuesheet/<cuesheet_uuid>")]
pub fn remove_cuesheet_from_playlist(
    uuid: String,
    cuesheet_uuid: String,
    conn: DbConn,
) -> QueryResult<Json<usize>> {
    playlists::remove_cuesheet_from_playlist(&uuid, &cuesheet_uuid, &conn).map(Json)
}

#[put("/v2/playlists/<uuid>/cuesheet/<cuesheet_uuid>")]
pub fn add_cuesheet_to_playlist(
    uuid: String,
    cuesheet_uuid: String,
    conn: DbConn,
) -> Result<Json<String>, Status> {
    match playlists::add_cuesheet_to_playlist(&uuid, &cuesheet_uuid, &conn) {
        Ok(s) => Ok(Json(s)),
        _ => Err(Status::BadRequest),
    }
}

#[delete("/v2/playlists/<uuid>")]
pub fn delete_playlist(uuid: String, conn: DbConn) -> QueryResult<Json<Playlist>> {
    playlists::delete_playlist(&uuid, &conn).map(Json)
}

#[put("/v2/playlists", format = "application/json", data = "<playlist>")]
pub fn create_playlist(playlist: Json<FormPlaylist>, conn: DbConn) -> QueryResult<Json<Playlist>> {
    let data = playlist.into_inner();
    let u = Uuid::new_v4().to_hyphenated().to_string();

    let p = PlaylistData {
        uuid: &u,
        name: &data.name,
    };

    playlists::create_playlist(&p, &conn).map(Json)
}

/*#[get("/v2/playlists/<id>")]
fn playlist_by_id(id: i32, conn: DbConn) -> QueryResult<Json<Playlist>> {
    playlists::playlist_by_id(&id, &conn).map(|playlist| Json(playlist))
}*/

#[get("/v2/playlists")]
pub fn get_playlists(conn: DbConn) -> Json<Vec<FullPlaylist>> {
    let mut lists: Vec<FullPlaylist> = vec![];
    for p in playlists::get_playlists(&conn).unwrap().into_iter() {
        let cuecards = playlists::get_cuecards(&p, &conn).unwrap();

        lists.push(FullPlaylist {
            id: p.id,
            uuid: p.uuid,
            name: p.name,
            cuecards,
        });
    }

    Json(lists)
}

#[get("/v2/cuecards/<uuid>")]
pub fn cuecard_content_by_uuid(
    uuid: String,
    conn: DbConn,
) -> Result<content::Html<String>, Status> {
    match cuecards::get_cuesheet_content(&uuid, &conn) {
        Ok(cuecard) => {
            let options = ComrakOptions {
                ext_tasklist: true,
                ext_footnotes: true,
                ext_table: true,
                hardbreaks: true,
                ext_tagfilter: true,
                unsafe_: true,
                ..ComrakOptions::default()
            };
            let markdown = markdown_to_html(&cuecard.content, &options);

            Ok(content::Html(markdown))
        }
        _ => Err(Status::NotFound),
    }
}

#[get("/v2/search/<query>")]
pub fn search_cuecards(query: String, conn: DbConn) -> QueryResult<Json<Vec<Cuecard>>> {
    cuecards::search_cuecards(&query, &conn).map(Json)
}

#[delete("/v2/events/<uuid>")]
pub fn delete_event(uuid: String, conn: DbConn) -> Result<Json<Event>, Status> {
    programming::delete_event(&uuid, &conn)
        .map(Json)
        .or_else(|_| Err(Status::NotFound))
}

#[get("/v2/events/<uuid>")]
pub fn event_by_uuid(uuid: String, conn: DbConn) -> Result<Json<Event>, Status> {
    programming::event_by_uuid(&uuid, &conn)
        .map(Json)
        .or_else(|_| Err(Status::NotFound))
}

#[get("/v2/events/<min_date>/<max_date>")]
pub fn get_events(
    conn: DbConn,
    min_date: String,
    max_date: String,
) -> Result<Json<Vec<Event>>, Status> {
    let start_date = DateTime::parse_from_rfc3339(min_date.as_str());
    let end_date = DateTime::parse_from_rfc3339(max_date.as_str());

    if start_date.is_err() {
        return Err(Status::BadRequest);
    }

    if end_date.is_err() {
        return Err(Status::BadRequest);
    }

    let start_date = start_date.unwrap();
    let end_date = end_date.unwrap();

    if start_date > end_date {
        return Err(Status::BadRequest);
    }

    programming::get_events(&conn, start_date.to_rfc3339(), end_date.to_rfc3339())
        .map(Json)
        .or_else(|_| Err(Status::BadRequest))
}

#[put("/v2/event", format = "application/json", data = "<event>")]
pub fn create_event(event: Json<FormEvent>, conn: DbConn) -> Result<Json<Event>, Status> {
    let data = event.into_inner();
    let u = Uuid::new_v4().to_hyphenated().to_string();

    let e = EventData {
        uuid: &u,
        name: &data.name,
        date_start: &data.date_start,
        date_end: &data.date_end,
        schedule: data.schedule,
        date_created: &data.date_created,
        date_modified: &data.date_modified,
    };

    let event = programming::create_event(&e, &conn);

    match event {
        Ok(event) => {
            let u = Uuid::new_v4().to_hyphenated().to_string();

            {
                let p = ProgramData {
                    uuid: &u,
                    notes: None,
                    event_id: &event.id,
                    date_created: &data.date_created,
                    date_modified: &data.date_modified,
                };

                p.create(&conn).unwrap();
            }

            Ok(Json(event))
        }
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/v2/event/program/<event_id>")]
pub fn get_program(event_id: i32, conn: DbConn) -> Result<Json<Option<Program>>, Status> {
    programming::program_by_event_id(event_id, &conn)
        .map(|p| match p {
            Some(p) => {
                let options = ComrakOptions {
                    ext_tasklist: true,
                    ..ComrakOptions::default()
                };
                let markdown =
                    markdown_to_html(p.notes.unwrap_or_else(|| "".to_string()).as_str(), &options);
                let program = Program {
                    id: p.id,
                    uuid: p.uuid,
                    notes: Some(markdown),
                    event_id: p.event_id,
                    date_created: p.date_created,
                    date_modified: p.date_modified,
                };
                Json(Some(program))
            }
            None => Json(None),
        })
        .or_else(|_| Err(Status::BadRequest))
}

#[get("/v2/tips/<program_id>")]
pub fn get_tips(program_id: i32, conn: DbConn) -> Result<Json<Vec<FullTip>>, Status> {
    programming::tips_by_program_id(program_id, &conn)
        .map(|tips| {
            let mut result: Vec<FullTip> = Vec::with_capacity(tips.len());
            for tip in tips.into_iter() {
                let cuecards =
                    programming::get_cuecards(&tip, &conn).unwrap_or_else(|_| Vec::new());

                let mut full_tip = FullTip::from(tip);
                full_tip.cuecards = cuecards;
                result.push(full_tip)
            }
            Json(result)
        })
        .or_else(|_| Err(Status::BadRequest))
}

#[put("/v2/tips", format = "application/json", data = "<tip>")]
pub fn create_tip(tip: Json<FormTip>, conn: DbConn) -> Result<Json<FullTip>, Status> {
    let data = tip.into_inner();
    let u = Uuid::new_v4().to_hyphenated().to_string();

    let tip_data = TipData {
        name: &data.name,
        uuid: &u,
        program_id: &data.program_id,
        date_start: &data.date_start,
        date_end: &data.date_end,
    };

    let result = programming::create_tip(&tip_data, &conn);

    match result {
        Ok(result) => Ok(Json(FullTip::from(result))),
        Err(_) => Err(Status::BadRequest),
    }
}

#[delete("/v2/tips/<tip_uuid>", format = "application/json")]
pub fn remove_tip(tip_uuid: String, conn: DbConn) -> Result<Json<()>, Status> {
    let tip = match cuer_database::tip_by_uuid(&tip_uuid, &conn) {
        Ok(tip) => tip,
        Err(_) => return Err(Status::NotFound),
    };

    let result = cuer_database::tip_delete(&tip, &conn);

    match result {
        Ok(_) => Ok(Json(())),
        Err(_) => Err(Status::BadRequest),
    }
}

#[put("/v2/tip_cuecard", format = "application/json", data = "<tip_cuecard>")]
pub fn create_tip_cuecard(
    tip_cuecard: Json<FormTipCuecard>,
    conn: DbConn,
) -> Result<Json<()>, Status> {
    let data = tip_cuecard.into_inner();

    let tip = match cuer_database::tip_by_uuid(&data.tip_uuid, &conn) {
        Ok(tip) => tip,
        Err(_) => return Err(Status::NotFound),
    };

    let cuecard = match cuer_database::cuecard_by_uuid(&data.cuecard_uuid, &conn) {
        Ok(cuecard) => cuecard,
        Err(_) => return Err(Status::NotFound),
    };

    let tip_cuecard_data = TipCuecardData {
        tip_id: &tip.id,
        cuecard_id: &cuecard.id,
        sort_order: &data.sort_order
    };

    let result = programming::create_tip_cuecard(&tip_cuecard_data, &conn);

    match result {
        Ok(_) => Ok(Json(())),
        Err(_) => Err(Status::BadRequest),
    }
}

#[post("/v2/tip_cuecard", format = "application/json", data = "<tip_cuecard>")]
pub fn update_tip_cuecard(
    tip_cuecard: Json<FormTipCuecard>,
    conn: DbConn,
) -> Result<Json<()>, Status> {
    let data = tip_cuecard.into_inner();

    let tip = match cuer_database::tip_by_uuid(&data.tip_uuid, &conn) {
        Ok(tip) => tip,
        Err(_) => return Err(Status::NotFound),
    };

    let cuecard = match cuer_database::cuecard_by_uuid(&data.cuecard_uuid, &conn) {
        Ok(cuecard) => cuecard,
        Err(_) => return Err(Status::NotFound),
    };

    let tip_cuecard_data = TipCuecardData {
        tip_id: &tip.id,
        cuecard_id: &cuecard.id,
        sort_order: &data.sort_order
    };

    let result = programming::update_tip_cuecard(&tip_cuecard_data, &conn);

    match result {
        Ok(_) => Ok(Json(())),
        Err(_) => Err(Status::BadRequest),
    }
}

#[delete(
    "/v2/tips/<tip_uuid>/cuecard/<cuecard_uuid>",
    format = "application/json"
)]
pub fn remove_tip_cuecard(
    tip_uuid: String,
    cuecard_uuid: String,
    conn: DbConn,
) -> Result<Json<()>, Status> {
    let tip = match cuer_database::tip_by_uuid(&tip_uuid, &conn) {
        Ok(tip) => tip,
        Err(_) => return Err(Status::NotFound),
    };

    let cuecard = match cuer_database::cuecard_by_uuid(&cuecard_uuid, &conn) {
        Ok(cuecard) => cuecard,
        Err(_) => return Err(Status::NotFound),
    };

    let tip_cuecard = match programming::get_tip_cuecard(tip.id, cuecard.id, &conn) {
        Ok(tip_cuecard) => tip_cuecard,
        Err(_) => return Err(Status::NotFound),
    };

    let tip_cuecard_data = TipCuecardData {
        tip_id: &tip_cuecard.tip_id,
        cuecard_id: &tip_cuecard.cuecard_id,
        sort_order: &tip_cuecard.sort_order
    };

    let result = programming::remove_tip_cuecard(&tip_cuecard_data, &conn);

    match result {
        Ok(_) => Ok(Json(())),
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/favicon.ico")]
pub fn favicon() -> io::Result<NamedFile> {
    NamedFile::open("public/favicon.ico")
}

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("public/index.html")
}

#[allow(unused_variables)]
#[get("/<something..>", rank=2)]
pub fn catchall(something: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open("public/index.html")
}

#[get("/static/<file..>")]
pub fn static_files(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new("public").join(file);
    NamedFile::open(path).ok()
}

#[get("/v2/audio/<file..>")]
pub fn audio_file(file: PathBuf) -> Option<NamedFile> {
    let file = format!("/home/music/collection/{}", file.to_str()?);
    let path = Path::new(&file);
    NamedFile::open(path).ok()
}
