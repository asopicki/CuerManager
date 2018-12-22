/**

This file contains all database related functions required for the programming of events.
This means the module is responsible for handling events, programs, tips and their realtionships.

**/


use diesel::prelude::*;
use cuer_database::models::{Event, EventData}; //, Program, ProgramData, Tip, TipData, TipCuecard, TipCuecardData};


/*pub enum EventError {
    DuplicateEvent,
    AddProgramError,
}*/

pub fn delete_event(uuid: &str, conn: &SqliteConnection) -> QueryResult<Event> {
    let e = event_by_uuid(uuid, conn).unwrap();

    e.delete(conn).unwrap();

    return Ok(e);
}

pub fn event_by_uuid(u: &str, conn: &SqliteConnection) -> QueryResult<Event> {
    use cuer_database::schema::events::dsl::*;
    events.filter(uuid.eq(u)).first::<Event>(conn)
}

pub fn get_events(conn: &SqliteConnection) -> QueryResult<Vec<Event>> {
    use cuer_database::schema::events::dsl::*;
    events.order(date_start.asc()).load::<Event>(conn)
}

pub fn create_event(event: &EventData, conn: &SqliteConnection) -> QueryResult<Event> {
    event.create(conn)
}