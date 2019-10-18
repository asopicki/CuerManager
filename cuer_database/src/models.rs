#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::cuecards;
use super::schema::events;
use super::schema::playlist_cuecards;
use super::schema::playlists;
use super::schema::programs;
use super::schema::tip_cuecards;
use super::schema::tips;
pub use diesel::prelude::*;
use diesel::{
    delete, insert_into, update, ExpressionMethods, QueryResult, RunQueryDsl, SqliteConnection,
};

#[derive(Clone, Queryable, Identifiable, QueryableByName, Debug, Serialize, Deserialize)]
#[table_name = "cuecards"]
pub struct Cuecard {
    pub id: i32,
    pub uuid: String,
    pub phase: String,
    pub rhythm: String,
    pub title: String,
    pub steplevel: String,
    pub difficulty: String,
    pub choreographer: String,
    pub meta: String,
    pub content: String,
    pub karaoke_marks: String,
    pub music_file: String
}

#[derive(AsChangeset, Debug)]
#[table_name = "cuecards"]
pub struct UpdateCuecard<'a> {
    pub uuid: &'a str,
    pub phase: &'a str,
    pub rhythm: &'a str,
    pub title: &'a str,
    pub steplevel: &'a str,
    pub difficulty: &'a str,
    pub choreographer: &'a str,
    pub meta: &'a str,
    pub content: &'a str,
    pub karaoke_marks: &'a str,
    pub music_file: &'a str
}

#[derive(Insertable, AsChangeset, Debug)]
#[table_name = "cuecards"]
pub struct CuecardData<'a> {
    pub uuid: &'a str,
    pub phase: &'a str,
    pub rhythm: &'a str,
    pub title: &'a str,
    pub steplevel: &'a str,
    pub difficulty: &'a str,
    pub choreographer: &'a str,
    pub meta: &'a str,
    pub content: &'a str,
    pub karaoke_marks: &'a str,
    pub music_file: &'a str
}

/*impl Cuecard {
    pub fn delete(&self, conn: &SqliteConnection) -> QueryResult<usize> {
        use crate::schema::cuecards::dsl::*;

        delete(cuecards).filter(id.eq(self.id)).execute(conn)
    }
}*/

impl<'a> CuecardData<'a> {
    pub fn update(&self, card: &Cuecard, conn: &SqliteConnection) -> QueryResult<usize> {
        update(card).set(self).execute(conn)
    }

    /// Inserts the cuecard into the database, or updates an existing one.
    pub fn create(&self, conn: &SqliteConnection) -> QueryResult<usize> {
        use crate::schema::cuecards::dsl::*;

        insert_into(cuecards).values(self).execute(conn)
    }
}

#[derive(Clone, Queryable, Identifiable, Debug, Serialize, Deserialize)]
pub struct Playlist {
    pub id: i32,
    pub uuid: String,
    pub name: String,
}

impl Playlist {
    pub fn delete(&self, conn: &SqliteConnection) -> QueryResult<usize> {
        use crate::schema::playlists::dsl::*;

        delete(playlists.filter(id.eq(self.id))).execute(conn)
    }
}

#[derive(Insertable, AsChangeset, Debug)]
#[table_name = "playlists"]
pub struct PlaylistData<'a> {
    pub uuid: &'a str,
    pub name: &'a str,
}

impl<'a> PlaylistData<'a> {
    pub fn update(&self, conn: &SqliteConnection) -> QueryResult<Playlist> {
        use crate::schema::playlists::dsl::*;
        update(playlists).set(self).execute(conn).unwrap();

        playlists.filter(uuid.eq(self.uuid)).get_result(conn)
    }

    /// Inserts the cuecard into the database, or updates an existing one.
    pub fn create(&self, conn: &SqliteConnection) -> QueryResult<Playlist> {
        use crate::schema::playlists::dsl::*;

        insert_into(playlists).values(self).execute(conn).unwrap();

        playlists.filter(uuid.eq(self.uuid)).get_result(conn)
    }
}

#[derive(
    Clone, Queryable, Identifiable, Associations, QueryableByName, Debug, Serialize, Deserialize,
)]
#[belongs_to(Playlist)]
#[belongs_to(Cuecard)]
#[table_name = "playlist_cuecards"]
pub struct PlaylistCuecard {
    pub id: i32,
    pub playlist_id: i32,
    pub cuecard_id: i32,
}

impl PlaylistCuecard {
    pub fn delete(&self, conn: &SqliteConnection) -> QueryResult<usize> {
        use crate::schema::playlist_cuecards::dsl::*;

        delete(playlist_cuecards)
            .filter(id.eq(self.id))
            .execute(conn)
    }
}

#[derive(Insertable, AsChangeset, Debug)]
#[table_name = "playlist_cuecards"]
pub struct NewPlaylistCuecard<'a> {
    pub playlist_id: &'a i32,
    pub cuecard_id: &'a i32,
}

impl<'a> NewPlaylistCuecard<'a> {
    /// Inserts the cuecard into the database
    pub fn create(&self, conn: &SqliteConnection) -> QueryResult<usize> {
        use crate::schema::playlist_cuecards::dsl::*;
        insert_into(playlist_cuecards).values(self).execute(conn)
    }
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Cardindex {
    pub rowid: i32,
    pub docid: i32,
    pub title: String,
    pub choreographer: String,
    pub meta: String,
    pub content: String,
}

#[derive(Clone, Queryable, Identifiable, QueryableByName, Debug, Serialize, Deserialize)]
#[table_name = "events"]
pub struct Event {
    pub id: i32,
    pub uuid: String,
    pub date_start: String,
    pub date_end: String,
    pub name: String,
    pub schedule: Option<String>,
    pub date_created: String,
    pub date_modified: String,
}

impl Event {
    pub fn delete(&self, conn: &SqliteConnection) -> QueryResult<usize> {
        use crate::schema::events::dsl::*;

        delete(events.filter(id.eq(self.id))).execute(conn)
    }
}

#[derive(Insertable, AsChangeset, Debug)]
#[table_name = "events"]
pub struct EventData<'a> {
    pub uuid: &'a str,
    pub name: &'a str,
    pub date_start: &'a str,
    pub date_end: &'a str,
    pub schedule: Option<&'a str>,
    pub date_created: &'a str,
    pub date_modified: &'a str,
}

impl<'a> EventData<'a> {
    pub fn update(&self, conn: &SqliteConnection) -> QueryResult<Event> {
        use crate::schema::events::dsl::*;
        update(events).set(self).execute(conn).unwrap();

        events.filter(uuid.eq(self.uuid)).get_result(conn)
    }

    pub fn create(&self, conn: &SqliteConnection) -> QueryResult<Event> {
        use crate::schema::events::dsl::*;

        insert_into(events).values(self).execute(conn).unwrap();

        events.filter(uuid.eq(self.uuid)).get_result(conn)
    }
}

#[derive(Clone, Queryable, Identifiable, QueryableByName, Debug, Serialize, Deserialize)]
#[table_name = "programs"]
pub struct Program {
    pub id: i32,
    pub uuid: String,
    pub notes: Option<String>,
    pub event_id: i32,
    pub date_created: String,
    pub date_modified: String,
}

#[derive(Insertable, AsChangeset, Debug)]
#[table_name = "programs"]
pub struct ProgramData<'a> {
    pub uuid: &'a str,
    pub notes: Option<&'a str>,
    pub event_id: &'a i32,
    pub date_created: &'a str,
    pub date_modified: &'a str,
}

impl<'a> ProgramData<'a> {
    pub fn update(&self, conn: &SqliteConnection) -> QueryResult<Program> {
        use crate::schema::programs::dsl::*;
        update(programs).set(self).execute(conn).unwrap();

        programs.filter(uuid.eq(self.uuid)).get_result(conn)
    }

    pub fn create(&self, conn: &SqliteConnection) -> QueryResult<Program> {
        use crate::schema::programs::dsl::*;

        insert_into(programs).values(self).execute(conn).unwrap();

        programs.filter(uuid.eq(self.uuid)).get_result(conn)
    }
}

#[derive(Clone, Queryable, Identifiable, QueryableByName, Debug, Serialize, Deserialize)]
#[table_name = "tips"]
pub struct Tip
<>
{
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub program_id: i32,
    pub date_start: String,
    pub date_end: String,
}

#[derive(Insertable, AsChangeset, Debug)]
#[table_name = "tips"]
pub struct TipData<'a> {
    pub uuid: &'a str,
    pub name: &'a str,
    pub program_id: &'a i32,
    pub date_start: &'a str,
    pub date_end: &'a str,
}

impl<'a> TipData<'a> {
    pub fn update(&self, conn: &SqliteConnection) -> QueryResult<Tip> {
        use crate::schema::tips::dsl::*;
        update(tips).set(self).execute(conn).unwrap();

        tips.filter(uuid.eq(self.uuid)).get_result(conn)
    }

    pub fn create(&self, conn: &SqliteConnection) -> QueryResult<Tip> {
        use crate::schema::tips::dsl::*;

        insert_into(tips).values(self).execute(conn).unwrap();

        tips.filter(uuid.eq(self.uuid)).get_result(conn)
    }
}

#[derive(
    Clone, Queryable, Identifiable, Associations, QueryableByName, Debug, Serialize, Deserialize,
)]
#[belongs_to(Tip)]
#[belongs_to(Cuecard)]
#[table_name = "tip_cuecards"]
pub struct TipCuecard {
    pub id: i32,
    pub tip_id: i32,
    pub cuecard_id: i32,
    pub sort_order: i32
}

#[derive(Insertable, AsChangeset, Debug)]
#[table_name = "tip_cuecards"]
pub struct TipCuecardData<'a> {
    pub tip_id: &'a i32,
    pub cuecard_id: &'a i32,
    pub sort_order: &'a i32
}

impl<'a> TipCuecardData<'a> {
    pub fn create(&self, conn: &SqliteConnection) -> QueryResult<usize> {
        use crate::schema::tip_cuecards::dsl::*;

        insert_into(tip_cuecards).values(self).execute(conn)
    }

    pub fn delete(&self, conn: &SqliteConnection) -> QueryResult<usize> {
        use crate::schema::tip_cuecards::dsl::*;

        delete(tip_cuecards.filter(tip_id.eq(self.tip_id).and(cuecard_id.eq(self.cuecard_id))))
            .execute(conn)
    }
}
