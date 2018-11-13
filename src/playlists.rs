use diesel::prelude::*;
use cuer_database::models::{Playlist, PlaylistCuecard, NewPlaylistCuecard, Cuecard, PlaylistData};
use cuer_database;

pub enum PlaylistsError {
	DuplicateCuecard,
	AddCuecardError,
}

pub fn add_cuesheet_to_playlist(u: &str, c_uuid: &str, conn: &SqliteConnection) -> Result<String, PlaylistsError> {
	use cuer_database::schema::cuecards::dsl::*;

	let p = playlist_by_uuid(u, conn).unwrap();

	let c = cuecards.filter(uuid.eq(c_uuid)).first::<Cuecard>(conn).unwrap();

	let pc = playlist_cuecard(&p.id, &c.id, conn);

	if pc.is_ok() {
		return Err(PlaylistsError::DuplicateCuecard);
	}

	let entry = NewPlaylistCuecard {
		playlist_id: &p.id,
		cuecard_id: &c.id
	};

	match entry.create(conn) {
		Ok(_i) => Ok(p.uuid),
		_ => Err(PlaylistsError::AddCuecardError)
	}
}

pub fn remove_cuesheet_from_playlist(u: &str, c_uuid: &str, conn: &SqliteConnection) -> QueryResult<usize> {
	let p = playlist_by_uuid(u, conn).unwrap();
	let c = cuer_database::cuecard_by_uuid(&c_uuid.to_string(), conn).unwrap();

	let pc = playlist_cuecard(&p.id, &c.id, conn).unwrap();
	return pc.delete(conn)
}

fn playlist_cuecard(i: &i32, c_id: &i32, conn: &SqliteConnection) -> QueryResult<PlaylistCuecard> {
	use cuer_database::schema::playlist_cuecards::dsl::*;
	playlist_cuecards.filter(playlist_id.eq(i).and(cuecard_id.eq(c_id)))
		.first::<PlaylistCuecard>(conn)
}

pub fn delete_playlist(uuid: &str, conn: &SqliteConnection) -> QueryResult<Playlist> {
	let p = playlist_by_uuid(uuid, conn).unwrap();

	p.delete(conn).unwrap();

	return Ok(p);
}

pub fn playlist_by_uuid(u: &str, conn: &SqliteConnection) -> QueryResult<Playlist> {
	use cuer_database::schema::playlists::dsl::*;
	playlists.filter(uuid.eq(u)).first::<Playlist>(conn)
}

pub fn get_playlists(conn: &SqliteConnection) -> QueryResult<Vec<Playlist>> {
	use cuer_database::schema::playlists::dsl::*;
	playlists.order(name.asc()).load::<Playlist>(conn)
}

pub fn create_playlist(playlist: &PlaylistData, conn: &SqliteConnection) -> QueryResult<Playlist> {
	playlist.create(conn)
}

pub fn get_cuecards(p: &Playlist, conn: &SqliteConnection) -> QueryResult<Vec<Cuecard>> {
	cuer_database::schema::playlist_cuecards::table.inner_join(cuer_database::schema::cuecards::table)
		.filter(cuer_database::schema::playlist_cuecards::columns::playlist_id.eq(p.id))
		.select(cuer_database::schema::cuecards::all_columns).load::<Cuecard>(conn)
}