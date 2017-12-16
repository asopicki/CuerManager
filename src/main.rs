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

#[cfg(test)]
mod tests;

mod documents;
mod playlists;
mod routes;
mod elastic;

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![
        routes::index,
        routes::static_files,
        routes::search_cuesheets,
        routes::cuesheet_by_id,
        routes::favicon,
        routes::get_playlists,
        routes::playlist_by_id,
        routes::create_playlist])
}

fn main() {
    rocket().launch();
}
