use diesel::prelude::*;
use cuer_database::models::{Cuecard};
use cuer_database;
use guards::DbConn;


pub fn search_cuecards(query: &String, conn: &DbConn) -> QueryResult<Vec<Cuecard>> {

	let has_search_prefix = query.contains(":");

	if has_search_prefix {
		let parts = query.splitn(2, ":").collect::<Vec<_>>();
		let mut iter = parts.iter();
		let (search_type, search_string) = (iter.next().unwrap(), iter.next().unwrap());

		match *search_type {
			"phase" => cuecards_by_phase(search_string, conn),
			"rhythm" => cuecards_by_rhythm(search_string, conn),
			_ => cuecards_meta_search(search_string, conn)
		}
	} else {
		cuecards_content_search(query, conn)
	}
}

pub fn get_cuesheet_content(u: &String, conn: &DbConn) -> QueryResult<Cuecard> {
	cuer_database::cuecard_by_uuid(u, conn)
}

fn cuecards_by_phase(p: &str, conn: &SqliteConnection) -> QueryResult<Vec<Cuecard>> {
	use cuer_database::schema::cuecards::dsl::*;

	cuecards.filter(phase.eq(p)).get_results(conn)
}

fn cuecards_by_rhythm(r: &str, conn: &SqliteConnection) -> QueryResult<Vec<Cuecard>> {
	use cuer_database::schema::cuecards::dsl::*;

	cuecards.filter(rhythm.eq(r)).get_results(conn)
}

fn cuecards_meta_search(q: &str, conn: &SqliteConnection) -> QueryResult<Vec<Cuecard>> {
	use cuer_database::schema::*;

	let ids = cardindex::table.select(cardindex::columns::docid)
		.filter(cuer_database::cd_match(cardindex::columns::content, q))
		.load::<i32>(conn).unwrap();

	return  cuecards::table.select(cuecards::all_columns).filter(cuecards::columns::id.eq_any(ids))
		.load::<Cuecard>(conn);
}

fn cuecards_content_search(q: &str, conn: &SqliteConnection) -> QueryResult<Vec<Cuecard>> {
	use cuer_database::schema::*;

	let ids = cardindex::table.select(cardindex::columns::docid)
		.filter(cuer_database::cd_match(cardindex::columns::content, q))
		.load::<i32>(conn).unwrap();

	return  cuecards::table.select(cuecards::all_columns).filter(cuecards::columns::id.eq_any(ids))
		.load::<Cuecard>(conn);
}

