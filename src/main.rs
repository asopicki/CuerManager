#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate comrak;
extern crate cuer_database;
extern crate dirs;
extern crate serde;
extern crate serde_json;
extern crate unescape;
extern crate uuid as uuidcrate;

#[cfg(test)]
mod tests;

mod cuecards;
mod playlists;
mod programming;
mod routes;

use rocket_contrib::databases::diesel;

#[database("sqlite_db")]
pub struct DbConn(diesel::SqliteConnection);

// The URL to the database, set via the `DATABASE_URL` environment variable.
//static DEFAULT_DATABASE_URL: &'static str = ".local/share/library.db";

fn rocket() -> rocket::Rocket {
    rocket::ignite().attach(DbConn::fairing()).mount(
        "/",
        routes![
            routes::index,
            routes::static_files,
            routes::search_cuecards,
            routes::cuecard_content_by_uuid,
            routes::favicon,
            routes::get_playlists,
            routes::create_playlist,
            routes::add_cuesheet_to_playlist,
            routes::delete_playlist,
            routes::remove_cuesheet_from_playlist,
            routes::get_events,
            routes::event_by_uuid,
            routes::delete_event,
            routes::create_event,
            routes::get_program,
            routes::get_tips,
            routes::create_tip,
            routes::remove_tip,
            routes::create_tip_cuecard,
            routes::remove_tip_cuecard
        ],
    )
}

fn main() {
    rocket().launch();
}
