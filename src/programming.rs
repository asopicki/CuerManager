/**

This file contains all database related functions required for the programming of events.
This means the module is responsible for handling events, programs, tips and their realtionships.

**/


use diesel::prelude::*;
use cuer_database::models::{Event, EventData, Program, Tip, TipData, Cuecard, TipCuecardData};


/*pub enum EventError {
    DuplicateEvent,
    AddProgramError,
}*/

pub fn delete_event(uuid: &str, conn: &SqliteConnection) -> QueryResult<Event> {
    let e = event_by_uuid(uuid, conn).unwrap();

    e.delete(conn).unwrap();

    Ok(e)
}

pub fn event_by_uuid(entry_uuid: &str, conn: &SqliteConnection) -> QueryResult<Event> {
    use cuer_database::schema::events::dsl::*;
    events.filter(uuid.eq(entry_uuid)).first::<Event>(conn)
}

pub fn get_events(conn: &SqliteConnection, min_date: String, max_date: String) -> QueryResult<Vec<Event>> {
    use cuer_database::schema::events::dsl::*;
    events
        .filter(date_start.ge(min_date))
        .filter(date_start.lt(max_date))
        .order(date_start.asc()).load::<Event>(conn)
}

pub fn create_event(event: &EventData, conn: &SqliteConnection) -> QueryResult<Event> {
    event.create(conn)
}

pub fn create_tip(tip: &TipData, conn: &SqliteConnection) -> QueryResult<Tip> {
    tip.create(conn)
}

pub fn program_by_event_id(program_event_id: i32, conn: &SqliteConnection) -> QueryResult<Option<Program>> {
    use cuer_database::schema::programs::dsl::*;
    programs.filter(event_id.eq(program_event_id)).first::<Program>(conn).optional()
}

pub fn tips_by_program_id(tip_program_id: i32, conn: &SqliteConnection) -> QueryResult<Vec<Tip>> {
    use cuer_database::schema::tips::dsl::*;

    tips.filter(program_id.eq(tip_program_id)).order(date_start.asc()).load::<Tip>(conn)
}

pub fn create_tip_cuecard(tip_cuecard: &TipCuecardData, conn: &SqliteConnection) -> QueryResult<usize> {
    tip_cuecard.create(conn)
}

pub fn remove_tip_cuecard(tip_cuecard: &TipCuecardData, conn: &SqliteConnection) -> QueryResult<usize> {
    tip_cuecard.delete(conn)
}

pub fn get_cuecards(tip: &Tip, conn: &SqliteConnection) -> QueryResult<Vec<Cuecard>> {
	cuer_database::schema::tip_cuecards::table.inner_join(cuer_database::schema::cuecards::table)
		.filter(cuer_database::schema::tip_cuecards::columns::tip_id.eq(tip.id))
		.select(cuer_database::schema::cuecards::all_columns).load::<Cuecard>(conn)
}