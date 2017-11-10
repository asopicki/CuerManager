#[derive(Serialize, Deserialize, Debug)]
pub struct CuesheetRef {
    id: String,
    title: String
}

impl CuesheetRef {
    fn new(id: String, title: String) -> CuesheetRef {
        CuesheetRef {
            id: id,
            title: title
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Playlist {
    id: String,
    name: String,
    cuesheets: Vec<CuesheetRef>
}

#[derive(Debug)]
pub enum PlaylistsError {
    NotFound,
}

impl Playlist {
    fn new() -> Playlist {
        Playlist {
            id: String::from("1234566"),
            name: String::from("Süddeutscher Rounddance Treff"),
            cuesheets: vec![
                CuesheetRef::new(String::from("AV6y5g38JrfaciJfLpaU"), String::from("All shook up")),
                CuesheetRef::new(String::from("AV6y5g17JrfaciJfLpaK"), String::from("By Candlelight (Aux Bougies)")),
                CuesheetRef::new(String::from("AV6y5gzzJrfaciJfLpaC"), String::from("Axel F")),
                CuesheetRef::new(String::from("AV6y5g7YJrfaciJfLpai"), String::from("Back to black")),
                CuesheetRef::new(String::from("AV6y5gtoJrfaciJfLpZr"), String::from("Calm after the storm")),
            ]
        }
    }
}

pub fn playlist_by_id(id: &str) -> Result<Playlist, PlaylistsError> {
    if id == "1234566" {
        return Ok(Playlist::new())
    } else {
        return Err(PlaylistsError::NotFound)
    }
}

pub fn get_playlists() -> Vec<Playlist> {
    return vec![Playlist::new()]
}