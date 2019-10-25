#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

extern crate base64;
extern crate chrono;
extern crate comrak;
extern crate cuer_database;
extern crate dirs;
extern crate duct;
extern crate serde;
extern crate serde_json;
extern crate unescape;
extern crate uuid as uuidcrate;

#[macro_use]
extern crate diesel_migrations;

#[cfg(test)]
mod tests;

mod cuecards;
mod guards;
mod programming;
mod routes;

use rocket::fairing::AdHoc;
use rocket_contrib::databases::diesel;

use guards::BackendConfig;

#[database("sqlite_db")]
pub struct DbConn(diesel::SqliteConnection);

embed_migrations!("../migrations");

// The URL to the database, set via the `DATABASE_URL` environment variable.
//static DEFAULT_DATABASE_URL: &'static str = ".local/share/library.db";

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount(
            "/",
            routes![
                routes::index,
                routes::static_files,
                routes::get_all_cuecards,
                routes::search_cuecards,
                routes::get_cuecard_by_uuid,
                routes::cuecard_content_by_uuid,
                routes::refresh_cuecards_library,
                routes::favicon,
                routes::get_events,
                routes::event_by_uuid,
                routes::delete_event,
                routes::create_event,
                routes::get_program,
                routes::get_program_notes,
                routes::update_program_notes,
                routes::get_tips,
                routes::create_tip,
                routes::remove_tip,
                routes::create_tip_cuecard,
                routes::update_tip_cuecard,
                routes::remove_tip_cuecard,
                routes::catchall,
                routes::audio_file,
                routes::set_marks,
                routes::check_migrations,
                routes::run_migrations,
                routes::get_all_tags,
                routes::get_tags,
                routes::add_tag,
                routes::remove_tag
            ],
        )
        .attach(AdHoc::on_attach("Backend Config", |rocket| {
            let music_files_dir = rocket
                .config()
                .get_str("music_files_dir")
                .unwrap_or("music_files")
                .to_string();

            let cuecards_lib_dir = rocket
                .config()
                .get_str("cuecards_lib_dir")
                .unwrap_or("cuecards")
                .to_string();

            let indexer_path = rocket
                .config()
                .get_str("indexer_path")
                .unwrap_or("cuecard_indexer")
                .to_string();

            let db_url = rocket
                .config()
                .get_str("library_db")
                .unwrap_or("library.db")
                .to_string();

            Ok(rocket.manage(BackendConfig {
                music_files_dir,
                cuecards_lib_dir,
                indexer_path,
                db_url,
            }))
        }))
}

fn main() {
    rocket().launch();
}
