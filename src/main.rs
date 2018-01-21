#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate rs_es;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate unescape;
extern crate comrak;
extern crate diesel;
extern crate r2d2_diesel;
extern crate r2d2;
extern crate uuid as uuidcrate;
extern crate cuer_database;

#[cfg(test)]
mod tests;

mod documents;
mod playlists;
mod routes;
mod elastic;
mod guards;
mod cuecards;

use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;

// An alias to the type for a pool of Diesel SQLite connections.
type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// The URL to the database, set via the `DATABASE_URL` environment variable.
static DATABASE_URL: &'static str = env!("DATABASE_URL");

/// Initializes a database pool.
fn init_pool() -> Pool {
	let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_URL);
	r2d2::Pool::new(manager).expect("db pool")
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().manage(init_pool())
	    .mount("/", routes![
            routes::index,
            routes::static_files,
            routes::search_cuesheets,
            routes::search_cuecards,
            routes::cuesheet_by_id,
            routes::favicon,
            routes::get_playlists,
            routes::playlist_by_id,
            routes::create_playlist,
            routes::add_cuesheet_to_playlist,
            routes::delete_playlist,
            routes::remove_cuesheet_from_playlist],)
}

fn main() {
    rocket().launch();
}
