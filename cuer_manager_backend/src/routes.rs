use crate::convert;
use crate::cuecards;
use crate::guards::BackendConfig;
use crate::programming;
use comrak::{markdown_to_html, ComrakOptions};
use cuer_database;
use cuer_database::models::Cuecard;
use cuer_database::models::{
    CuecardData, Event, EventData, Program, ProgramData, Tag, Tip, TipCuecard, TipCuecardData,
    TipData,
};
use log::{error, info};
use uuidcrate::Uuid;

use std::convert::From;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile;

use rocket::data::Data;
use rocket::http::Status;
use rocket::response::{content, NamedFile};
use rocket::State;
use rocket_contrib::json::Json;

use chrono::prelude::*;

use duct::cmd;

use base64::decode;

use super::DbConn;
use diesel::QueryResult;

use diesel_migrations::{any_pending_migrations, run_pending_migrations};

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
    tip_cuecards: Vec<TipCuecard>,
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
            tip_cuecards: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FormUpdateTip {
    uuid: String,
    name: String,
    program_id: i32,
    date_start: String,
    date_end: String,
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
    cued_at: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FormCuecardMarks {
    karaoke_marks: String,
}

#[derive(Serialize, Deserialize)]
pub struct FormTag {
    tag: String,
}

#[derive(Serialize, Deserialize)]
pub struct FormNotes {
    notes: String,
    date_modified: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FormMetaData {
    choreographer: String,
    phase: String,
    difficulty: Option<String>,
    rhythm: String,
    plusfigures: Option<String>,
    steplevel: Option<String>,
    music: Option<String>,
    music_file: Option<String>,
}

#[get("/v2/cuecards/<uuid>/content")]
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

#[get("/v2/cuecards/<uuid>")]
pub fn get_cuecard_by_uuid(uuid: String, conn: DbConn) -> Result<Json<Cuecard>, Status> {
    match cuer_database::cuecard_by_uuid(&uuid, &conn) {
        Ok(cuecard) => Ok(Json(cuecard)),
        Err(_) => Err(Status::NotFound),
    }
}

#[post(
    "/v2/cuecards/<uuid>/marks",
    format = "application/json",
    data = "<marks>"
)]
pub fn set_marks(uuid: String, marks: Json<FormCuecardMarks>, conn: DbConn) -> Result<(), Status> {
    let data = marks.into_inner();

    let cuecard = match cuer_database::cuecard_by_uuid(&uuid, &conn) {
        Ok(cuecard) => cuecard,
        Err(_) => return Err(Status::NotFound),
    };

    match programming::set_marks(cuecard.id, &data.karaoke_marks, &conn) {
        Ok(_) => Ok(()),
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/v2/cuecards/<uuid>/metadata")]
pub fn get_cuecard_metadata(
    uuid: String,
    conn: DbConn,
    config: State<BackendConfig>,
) -> Result<Json<FormMetaData>, Status> {
    let cuecard = match cuer_database::cuecard_by_uuid(&uuid, &conn) {
        Ok(cuecard) => cuecard,
        Err(_) => return Err(Status::NotFound),
    };

    let base_path = PathBuf::from(&config.cuecards_lib_dir);

    let path = base_path
        .join(PathBuf::from(cuecard.file_path))
        .with_extension(".meta.json");

    if path.exists() {
        match serde_json::from_str::<FormMetaData>(&std::fs::read_to_string(path).unwrap()) {
            Ok(metadata) => Ok(Json(metadata)),
            Err(err) => {
                error!("Error reading metadata file: {:?}", err);
                Err(Status::BadRequest)
            }
        }
    } else {
        Ok(Json(FormMetaData::default()))
    }
}

#[post(
    "/v2/cuecards/<uuid>/metadata",
    format = "application/json",
    data = "<metadata>"
)]
pub fn set_cuecard_metadata(
    uuid: String,
    metadata: Json<FormMetaData>,
    conn: DbConn,
    config: State<BackendConfig>,
) -> Result<(), Status> {
    let data = metadata.into_inner();

    let cuecard = match cuer_database::cuecard_by_uuid(&uuid, &conn) {
        Ok(cuecard) => cuecard,
        Err(_) => return Err(Status::NotFound),
    };

    let base_path = PathBuf::from(&config.cuecards_lib_dir);

    let path = base_path
        .join(PathBuf::from(&cuecard.file_path))
        .with_extension(".meta.json");

    let serialized_data = match serde_json::to_string(&data) {
        Ok(m) => m,
        Err(err) => {
            error!("Error converting metadata: {:?}", err);
            return Err(Status::BadRequest);
        }
    };

    match std::fs::write(&path, &serialized_data) {
        Ok(_) => (),
        Err(error) => {
            error!("Error writing metadata to file {:?}: {:?}", path, error);
            return Err(Status::BadRequest);
        }
    };

    let time = Utc::now();

    let cuecard_data = CuecardData {
        uuid: &cuecard.uuid,
        phase: &data.phase,
        rhythm: &data.rhythm,
        title: &cuecard.title,
        steplevel: &data.steplevel.unwrap_or_default(),
        difficulty: &data.difficulty.unwrap_or_default(),
        choreographer: &data.choreographer,
        meta: &serialized_data,
        content: &cuecard.content,
        karaoke_marks: &cuecard.karaoke_marks,
        music_file: &data.music_file.unwrap_or_default(),
        file_path: &cuecard.file_path,
        date_created: &cuecard.date_created,
        date_modified: &time.format("%FT%T%.3fZ").to_string(),
    };

    match cuecard_data.update(&cuecard, &conn) {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("Error saving metadata to database: {:?}", err);
            Err(Status::BadRequest)
        }
    }
}

#[post("/v2/cuecards/<uuid>/cued_at")]
pub fn cued_at(uuid: String, conn: DbConn) -> Result<(), Status> {
    let cuecard = match cuer_database::cuecard_by_uuid(&uuid, &conn) {
        Ok(cuecard) => cuecard,
        Err(_) => return Err(Status::NotFound),
    };

    let time = Utc::now();

    let timestamp = time.format("%FT%T").to_string();

    match cuecards::get_tip_cuecard_to_current_event(&cuecard, &timestamp, &conn) {
        Ok(tip_cuecards) => {
            for tip_cuecard in tip_cuecards.iter() {
                let tip_cuecard_data = TipCuecardData {
                    tip_id: &tip_cuecard.tip_id,
                    cuecard_id: &cuecard.id,
                    sort_order: &tip_cuecard.sort_order,
                    cued_at: Some(time.format("%FT%T%.3fZ").to_string()),
                };

                match programming::update_tip_cuecard(&tip_cuecard_data, &conn) {
                    Ok(_) => (),
                    Err(_) => return Err(Status::BadRequest),
                }
            }
            Ok(())
        }
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/v2/cuecards")]
pub fn get_all_cuecards(conn: DbConn) -> Result<Json<Vec<Cuecard>>, Status> {
    match cuecards::get_all(&conn) {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(Status::BadRequest),
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
                    event_id: event.id,
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
        .or_else(|_| Err(Status::NotFound))
}

#[get("/v2/event/<event_id>/program/notes")]
pub fn get_program_notes(event_id: i32, conn: DbConn) -> Result<String, Status> {
    match programming::program_by_event_id(event_id, &conn) {
        Ok(p) => match p {
            Some(p) => Ok(p.notes.unwrap_or_else(|| "".to_owned())),
            None => Ok("".to_owned()),
        },
        Err(_) => Err(Status::NotFound),
    }
}

#[post(
    "/v2/program/<program_id>/notes",
    format = "application/json",
    data = "<notes>"
)]
pub fn update_program_notes(
    program_id: i32,
    notes: Json<FormNotes>,
    conn: DbConn,
) -> Result<String, Status> {
    let program = programming::get_program_by_id(program_id, &conn);

    let data = notes.into_inner();

    match program {
        Ok(p) => {
            let program_data = ProgramData {
                uuid: &p.uuid,
                notes: Some(&data.notes),
                event_id: p.event_id,
                date_created: &p.date_created,
                date_modified: &data.date_modified,
            };

            match program_data.update(&conn) {
                Ok(_) => {
                    let options = ComrakOptions {
                        ext_tasklist: true,
                        ..ComrakOptions::default()
                    };
                    Ok(markdown_to_html(&data.notes.as_str(), &options))
                }
                Err(_) => Err(Status::BadRequest),
            }
        }
        Err(_) => Err(Status::NotFound),
    }
}

#[get("/v2/tips/<program_id>")]
pub fn get_tips(program_id: i32, conn: DbConn) -> Result<Json<Vec<FullTip>>, Status> {
    programming::tips_by_program_id(program_id, &conn)
        .map(|tips| {
            let mut result: Vec<FullTip> = Vec::with_capacity(tips.len());
            for tip in tips.into_iter() {
                let cuecards =
                    programming::get_cuecards(&tip, &conn).unwrap_or_else(|_| Vec::new());

                let tip_cuecards =
                    programming::get_tip_cuecards(&tip, &conn).unwrap_or_else(|_| Vec::new());

                let mut full_tip = FullTip::from(tip);
                full_tip.cuecards = cuecards;
                full_tip.tip_cuecards = tip_cuecards;
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

#[post("/v2/tips", format = "application/json", data = "<tip>")]
pub fn update_tip(tip: Json<FormUpdateTip>, conn: DbConn) -> Result<Json<Tip>, Status> {
    let data = tip.into_inner();

    let tip = match programming::get_tip_by_uuid(&data.uuid, &conn) {
        Ok(tip) => tip,
        Err(_) => return Err(Status::NotFound),
    };

    let tip_data = TipData {
        uuid: &tip.uuid,
        name: &data.name,
        program_id: &tip.program_id,
        date_start: &data.date_start,
        date_end: &data.date_end,
    };

    match tip_data.update(&conn) {
        Ok(tip) => Ok(Json(tip)),
        Err(_err) => Err(Status::BadRequest),
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
        sort_order: &data.sort_order,
        cued_at: None,
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
        sort_order: &data.sort_order,
        cued_at: data.cued_at,
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
        sort_order: &tip_cuecard.sort_order,
        cued_at: tip_cuecard.cued_at,
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
#[get("/<something..>", rank = 2)]
pub fn catchall(something: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open("public/index.html")
}

#[get("/static/<file..>")]
pub fn static_files(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new("public").join(file);
    NamedFile::open(path).ok()
}

#[derive(Serialize, Deserialize)]
pub struct FormFilename {
    filename: String,
}

#[post("/v2/audio", format = "application/json", data = "<filedata>")]
pub fn audio_file(filedata: Json<FormFilename>, config: State<BackendConfig>) -> Option<NamedFile> {
    let filedata = filedata.into_inner();

    let file_name = match String::from_utf8(decode(&filedata.filename).unwrap()) {
        Ok(s) => s,
        Err(_) => return None,
    };

    let file_path = Path::new(&file_name);

    let path = Path::new(&config.music_files_dir).join(file_path);

    NamedFile::open(path).ok()
}

#[post("/v2/cuecards/refresh")]
pub fn refresh_cuecards_library(config: State<BackendConfig>) -> Result<(), Status> {
    let cmd = cmd!(
        String::from(&config.indexer_path),
        "--database",
        String::from(&config.db_url),
        String::from(&config.cuecards_lib_dir)
    )
    .env("DATABASE_URL", String::from(&config.db_url));

    match cmd.run() {
        Ok(_) => Ok(()),
        Err(_) => Err(Status::BadRequest),
    }
}

#[post("/v2/migrations/run")]
pub fn run_migrations(conn: DbConn) -> Result<Json<bool>, Status> {
    match run_pending_migrations(&conn.0) {
        Ok(()) => Ok(Json(true)),
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/v2/migrations/check")]
pub fn check_migrations(conn: DbConn) -> Result<Json<bool>, Status> {
    match any_pending_migrations(&conn.0) {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/v2/tags")]
pub fn get_all_tags(conn: DbConn) -> Result<Json<Vec<Tag>>, Status> {
    match cuecards::get_all_tags(&conn) {
        Ok(tags) => Ok(Json(tags)),
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/v2/cuecards/<uuid>/tags")]
pub fn get_tags(uuid: String, conn: DbConn) -> Result<Json<Vec<Tag>>, Status> {
    let cuecard = match cuer_database::cuecard_by_uuid(&uuid, &conn) {
        Ok(cuecard) => cuecard,
        Err(_) => return Err(Status::NotFound),
    };

    match cuecards::get_tags(&cuecard, &conn) {
        Ok(tags) => Ok(Json(tags)),
        Err(_) => Err(Status::NotFound),
    }
}

#[post(
    "/v2/cuecards/<uuid>/tags",
    format = "application/json",
    data = "<tagdata>"
)]
pub fn add_tag(uuid: String, tagdata: Json<FormTag>, conn: DbConn) -> Result<(), Status> {
    let cuecard = match cuer_database::cuecard_by_uuid(&uuid, &conn) {
        Ok(cuecard) => cuecard,
        Err(_) => return Err(Status::NotFound),
    };

    let data = tagdata.into_inner();

    match cuecards::get_tag_by_name(&data.tag, &conn) {
        Ok(tag) => {
            if !cuecards::tag_associated(&tag, &cuecard, &conn) {
                match cuecards::add_tag_to_cuecard(&tag, &cuecard, &conn) {
                    Ok(_) => return Ok(()),
                    Err(_) => return Err(Status::BadRequest),
                }
            }
        }
        Err(_) => {
            let result = cuecards::add_new_tag(&data.tag, &conn);

            match result {
                Ok(new_tag) => match cuecards::add_tag_to_cuecard(&new_tag, &cuecard, &conn) {
                    Ok(_) => return Ok(()),
                    Err(_) => return Err(Status::BadRequest),
                },
                Err(_) => return Err(Status::BadRequest),
            }
        }
    }

    Ok(())
}

#[delete("/v2/cuecards/<uuid>/tag/<tag>")]
pub fn remove_tag(uuid: String, tag: String, conn: DbConn) -> Result<(), Status> {
    let cuecard = match cuer_database::cuecard_by_uuid(&uuid, &conn) {
        Ok(cuecard) => cuecard,
        Err(_) => return Err(Status::NotFound),
    };

    match cuecards::get_tag_by_name(&tag, &conn) {
        Ok(tag) => {
            if cuecards::tag_associated(&tag, &cuecard, &conn) {
                match cuecards::remove_tag_from_cuecard(&tag, &cuecard, &conn) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(Status::BadRequest),
                }
            } else {
                Ok(())
            }
        }
        Err(_) => Err(Status::BadRequest),
    }
}

pub struct MarkdownFile(NamedFile);

impl<'r> rocket::response::Responder<'r> for MarkdownFile {
    fn respond_to(self, req: &rocket::request::Request) -> rocket::response::Result<'r> {
        rocket::response::Response::build_from(self.0.respond_to(req)?)
            .raw_header("Content-Type", "text/markdown")
            .raw_header(
                "Content-Disposition",
                "attachment; filename=\"converted.md\"",
            )
            .ok()
    }
}

#[post(
    "/v2/convert/odt",
    format = "application/octet-stream",
    data = "<data>"
)]
pub fn convert_odt_file(data: Data) -> Result<MarkdownFile, Status> {
    let src_file = tempfile::NamedTempFile::new().unwrap();
    let file = std::fs::File::create(&src_file).unwrap();

    let mut writer = std::io::BufWriter::new(&file);
    match data.stream_to(&mut writer) {
        Ok(size) => info!("Saved {} bytes to temporary file", size),
        Err(error) => {
            error!("Error writing to temporary file: {:?}", error);
            return Err(Status::BadRequest);
        }
    }
    match writer.flush() {
        Ok(_) => (),
        Err(err) => {
            error!("Error flushing output file: {:?}", err);
            return Err(Status::BadRequest);
        }
    }

    let file = std::fs::File::open(src_file).unwrap();
    let mut reader = std::io::BufReader::new(file);

    let target = tempfile::Builder::new().suffix(".md").tempfile().unwrap();
    let target_file = std::fs::File::create(&target).unwrap();
    let mut writer = std::io::BufWriter::new(target_file);

    match convert::convert_to_markdown(&mut reader, &mut writer) {
        Ok(_) => match NamedFile::open(target) {
            Ok(file) => Ok(MarkdownFile(file)),
            Err(_) => Err(Status::BadRequest),
        },
        Err(_) => Err(Status::BadRequest),
    }
}
