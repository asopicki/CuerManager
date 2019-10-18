#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod models;
pub mod schema;

use crate::models::{Cuecard, Playlist, Tip};
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::{delete, sql_query};

pub fn establish_connection(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn playlist_by_uuid(u: &str, connection: &SqliteConnection) -> QueryResult<Playlist> {
    use crate::schema::playlists::dsl::*;

    playlists.filter(uuid.eq(u)).first::<Playlist>(connection)
}

pub fn cuecard_by_uuid(u: &str, connection: &SqliteConnection) -> QueryResult<Cuecard> {
    use crate::schema::cuecards::dsl::*;

    cuecards.filter(uuid.eq(u)).first::<Cuecard>(connection)
}

pub fn tip_by_uuid(u: &str, connection: &SqliteConnection) -> QueryResult<Tip> {
    use crate::schema::tips::dsl::*;

    tips.filter(uuid.eq(u)).first::<Tip>(connection)
}

pub fn tip_delete(t: &Tip, connection: &SqliteConnection) -> QueryResult<usize> {
    use crate::schema::tips::dsl::*;

    sql_query("PRAGMA foreign_keys = 1").execute(connection)?;

    delete(tips.filter(id.eq(t.id))).execute(connection)
}

diesel_infix_operator!(CdMatch, " MATCH ");

// Normally you would put this on a trait instead
pub fn cd_match<T, U>(left: T, right: U) -> CdMatch<T, U::Expression>
where
    T: Expression,
    U: AsExpression<T::SqlType>,
{
    CdMatch::new(left, right.as_expression())
}

//let users_with_name = users.select(id).filter(cd_match(name, "Sean"));
