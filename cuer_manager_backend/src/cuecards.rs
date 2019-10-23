use cuer_database;
use cuer_database::models::{Cuecard, Tag, TagData, CuecardTag, CuecardTagData};
use diesel::prelude::*;

use super::DbConn;

type DBConnection = SqliteConnection;

pub fn add_new_tag(name: &str, conn: &DBConnection) -> QueryResult<Tag> {
    let data = TagData{tag: name};

    data.create(conn)
}

pub fn tag_associated(tag: &Tag, cuecard: &Cuecard, conn: &DBConnection) -> bool {
    use cuer_database::schema::cuecard_tags::dsl::*;

    match cuecard_tags
        .filter(cuecard_id.eq(cuecard.id))
        .filter(tag_id.eq(tag.id))
        .first::<CuecardTag>(conn) {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn add_tag_to_cuecard(tag: &Tag, cuecard: &Cuecard, conn: &DBConnection) -> QueryResult<usize> {
    let data = CuecardTagData{tag_id: tag.id, cuecard_id: cuecard.id};

    data.create(conn)
}

pub fn remove_tag_from_cuecard(tag: &Tag, cuecard: &Cuecard, conn: &DBConnection) -> QueryResult<usize> {
    let data = CuecardTagData{tag_id: tag.id, cuecard_id: cuecard.id};

    data.delete(conn)
}

pub fn get_tag_by_name(name: &str, conn: &DBConnection) -> QueryResult<Tag> {
    use cuer_database::schema::tags::dsl::*;

    tags.filter(tag.eq(name)).first::<Tag>(conn)
}

pub fn get_all_tags(conn: &DBConnection) -> QueryResult<Vec<Tag>> {
    use cuer_database::schema::tags::dsl::*;

    tags.load(conn)
}

pub fn get_tags(cuecard: &Cuecard, conn: &DBConnection) -> QueryResult<Vec<Tag>> {
    cuer_database::schema::cuecard_tags::table
        .inner_join(cuer_database::schema::tags::table)
        .filter(cuer_database::schema::cuecard_tags::columns::cuecard_id.eq(cuecard.id))
        .select(cuer_database::schema::tags::all_columns)
        .load(conn)
}

pub fn get_all(conn: &DBConnection) -> QueryResult<Vec<Cuecard>> {
     use cuer_database::schema::cuecards::dsl::*;

     cuecards.load(conn)
}

pub fn search_cuecards(query: &str, conn: &DbConn) -> QueryResult<Vec<Cuecard>> {
    let has_search_prefix = query.contains(':');

    if has_search_prefix {
        let parts = query.splitn(2, ':').collect::<Vec<_>>();
        let mut iter = parts.iter();
        let (search_type, search_string) = (iter.next().unwrap(), iter.next().unwrap());

        match *search_type {
            "phase" => cuecards_by_phase(search_string, conn),
            "rhythm" => cuecards_by_rhythm(search_string, conn),
            _ => cuecards_meta_search(search_string, conn),
        }
    } else {
        cuecards_content_search(query, conn)
    }
}

pub fn get_cuesheet_content(u: &str, conn: &DbConn) -> QueryResult<Cuecard> {
    cuer_database::cuecard_by_uuid(u, conn)
}

fn cuecards_by_phase(p: &str, conn: &DBConnection) -> QueryResult<Vec<Cuecard>> {
    use cuer_database::schema::cuecards::dsl::*;

    cuecards.filter(phase.eq(p)).get_results(conn)
}

fn cuecards_by_rhythm(r: &str, conn: &DBConnection) -> QueryResult<Vec<Cuecard>> {
    use cuer_database::schema::cuecards::dsl::*;

    cuecards.filter(rhythm.eq(r)).get_results(conn)
}

fn cuecards_meta_search(q: &str, conn: &DBConnection) -> QueryResult<Vec<Cuecard>> {
    use cuer_database::schema::*;

    let ids = cardindex::table
        .select(cardindex::columns::docid)
        .filter(cuer_database::cd_match(cardindex::columns::content, q))
        .load::<i32>(conn)
        .unwrap();

    cuecards::table
        .select(cuecards::all_columns)
        .filter(cuecards::columns::id.eq_any(ids))
        .load::<Cuecard>(conn)
}

fn cuecards_content_search(q: &str, conn: &DBConnection) -> QueryResult<Vec<Cuecard>> {
    use cuer_database::schema::*;

    let ids = cardindex::table
        .select(cardindex::columns::docid)
        .filter(cuer_database::cd_match(cardindex::columns::content, q))
        .load::<i32>(conn)
        .unwrap();

    cuecards::table
        .select(cuecards::all_columns)
        .filter(cuecards::columns::id.eq_any(ids))
        .load::<Cuecard>(conn)
}
