use diesel::prelude::*;
use cuer_database::models::{Playlist, PlaylistCuecard, NewPlaylistCuecard, Cuecard, PlaylistData};
use cuer_database;
use guards::DbConn;

const MAX_RESULTS: i64 = 200;

pub enum PlaylistsError {
	DuplicateCuecard,
	AddCuecardERror,
}

pub fn add_cuesheet_to_playlist(i: &i32, c_id: &i32, conn: &SqliteConnection) -> Result<usize, PlaylistsError> {
	use cuer_database::schema::cuecards::dsl::*;

	let p = playlist_by_id(i, conn).unwrap();

	let c = cuecards.filter(id.eq(c_id)).first::<Cuecard>(conn).unwrap();

	let pc = playlist_cuecard(i, c_id, conn);

	if pc.is_ok() {
		return Err(PlaylistsError::DuplicateCuecard);
	}

	let entry = NewPlaylistCuecard {
		playlist_id: &p.id,
		cuecard_id: &c.id
	};

	match entry.create(conn) {
		Ok(i) => Ok(i),
		_ => Err(PlaylistsError::AddCuecardERror)
	}
}

pub fn remove_cuesheet_from_playlist(i: &i32, c_id: &i32, conn: &SqliteConnection) -> QueryResult<usize> {
	let p = playlist_cuecard(i, c_id, conn).unwrap();
	p.delete(conn)

}

fn playlist_cuecard(i: &i32, c_id: &i32, conn: &SqliteConnection) -> QueryResult<PlaylistCuecard> {
	use cuer_database::schema::playlist_cuecards::dsl::*;
	playlist_cuecards.filter(playlist_id.eq(i).and(cuecard_id.eq(c_id)))
		.first::<PlaylistCuecard>(conn)
}

pub fn delete_playlist(id: &i32, conn: &DbConn) -> QueryResult<usize> {
	let p = playlist_by_id(id, conn).unwrap();

	return p.delete(conn);
}

pub fn playlist_by_id(i: &i32, conn: &SqliteConnection) -> QueryResult<Playlist> {
	use cuer_database::schema::playlists::dsl::*;
	playlists.filter(id.eq(i)).first::<Playlist>(conn)
}

pub fn playlist_by_uuid(u: &String, conn: &SqliteConnection) -> QueryResult<Playlist> {
	use cuer_database::schema::playlists::dsl::*;
	playlists.filter(uuid.eq(u)).first::<Playlist>(conn)
}

pub fn get_playlists(conn: &SqliteConnection) -> QueryResult<Vec<Playlist>> {
	use cuer_database::schema::playlists::dsl::*;
	playlists.order(name.asc()).limit(MAX_RESULTS).load::<Playlist>(conn)
}

pub fn create_playlist(playlist: &PlaylistData, conn: &SqliteConnection) -> QueryResult<Playlist> {
	playlist.create(conn)
}

pub fn get_cuecards(p: &Playlist, conn: &SqliteConnection) -> QueryResult<Vec<Cuecard>> {
	cuer_database::schema::playlist_cuecards::table.inner_join(cuer_database::schema::cuecards::table)
		.filter(cuer_database::schema::playlist_cuecards::columns::playlist_id.eq(p.id))
		.select(cuer_database::schema::cuecards::all_columns).load::<Cuecard>(conn)
}

/*fn save_playlist(playlist: &Playlist)  {

}*/