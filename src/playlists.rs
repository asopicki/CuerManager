use std::ops::Deref;
use std::boxed::Box;

use serde_json::Value;

use rs_es::query::Query;
use rs_es::operations::search;
use rs_es::operations::get::GetResult;
use elastic::{PLAYLIST_INDEX, PLAYLIST_TYPE, get_client};

use documents::get_cuesheet;
const MAX_RESULTS: u64 = 200;

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Playlist {
    id: String,
    name: String,
    cuesheets: Vec<CuesheetRef>
}

#[derive(Debug)]
pub enum PlaylistsError {
    NotFound,
	InvalidPlaylist,
	SearchError,
	SaveFailed
}

impl Playlist {
    fn new() -> Playlist {
        Playlist {
            id: String::from("1234566"),
            name: String::from(""),
            cuesheets: vec![]
        }
    }

	fn from_value(obj: &Value, id: String) -> Option<Playlist> {
		let mut playlist = Playlist::new();
		playlist.id = id.clone();

		playlist.name = String::from(obj.get("name").unwrap_or(&Value::from("")).to_string()).trim_matches('"').to_string();

		let cuesheets: Vec<CuesheetRef> = vec![];
		let default = vec![];
		let values = obj.get("cuesheets").unwrap().as_array().unwrap_or(&default);

		playlist.cuesheets = cuesheets;
		playlist.add_cuesheets(values);

		Some(playlist)
	}

	fn from(obj: &Option<Box<Value>>, id: String) -> Option<Playlist> {
		match obj {
			&Some(ref v) => {
				let mut playlist = Playlist::new();
				playlist.id = id.clone();

				match v.deref() {
					&Value::Object(ref obj) => {

						playlist.name = String::from(obj.get("name").unwrap_or(&Value::from("")).to_string()).trim_matches('"').to_string();

						let mut cuesheets: Vec<CuesheetRef> = vec![];
						let default = vec![];
						let values = obj.get("cuesheets").unwrap().as_array().unwrap_or(&default);

						playlist.cuesheets = cuesheets;

						playlist.add_cuesheets(values);

						Some(playlist)
					},
					_ => None
				}

			},
			_ => None,
		}
	}

	fn has_cuesheet(&self, cuesheet_id: &str) -> bool {
		let iter = self.cuesheets.iter();

		for cuesheet in  iter {
			if cuesheet.id == cuesheet_id {
				return true;
			}
		}

		return false;
	}

	fn add_cuesheet(&mut self, cuesheet: CuesheetRef)  {
		self.cuesheets.push(cuesheet);
	}

	fn add_cuesheets(&mut self, values: &Vec<Value>) {
		for value in values {
			let cuesheet_ref= CuesheetRef::new(
				String::from(value.get("id").unwrap_or(&Value::from("")).to_string()).trim_matches('"').to_string(),
				String::from(value.get("title").unwrap_or(&Value::from("")).to_string()).trim_matches('"').to_string(),
			);
			self.add_cuesheet(cuesheet_ref);
		}
	}

	fn remove_cuesheet(&mut self, cuesheet_id: &str) {
		self.cuesheets.retain(|ref cuesheet: &CuesheetRef| cuesheet.id != cuesheet_id);
	}
}

pub fn add_cuesheet_to_playlist(id: &str, cuesheet_id: &str) -> Result<Playlist, PlaylistsError> {

	match playlist_by_id(id)  {
		Ok(mut playlist) => {

			if playlist.has_cuesheet(cuesheet_id) {
				return Ok(playlist);
			}

			let result = get_cuesheet(cuesheet_id);

			match result  {
				Ok(cuesheet) => {
					let cuesheet_ref = CuesheetRef::new((*cuesheet.id()).clone(), (*cuesheet.title()).clone());
					playlist.add_cuesheet(cuesheet_ref);

					return match save_playlist(&playlist)  {
						Ok(_) => Ok(playlist),
						Err(e) => Err(e)
					}
				}
				Err(_) => Err(PlaylistsError::SaveFailed)
			}
		},
		Err(_) => Err(PlaylistsError::NotFound)
	}
}

pub fn remove_cuesheet_from_playlist(id: &str, cuesheet_id: &str) -> Result<Playlist, PlaylistsError> {
	match playlist_by_id(id)  {
		Ok(mut playlist) => {
			playlist.remove_cuesheet(cuesheet_id);

			return match save_playlist(&playlist) {
				Ok(_) => Ok(playlist),
				Err(e) => Err(e)
			}
		},
		Err(_) => Err(PlaylistsError::NotFound)
	}
}

pub fn delete_playlist(id: &str) -> Result<bool, PlaylistsError> {
	let mut client = get_client().unwrap();

	let result = client.delete(PLAYLIST_INDEX, PLAYLIST_TYPE, id).send();

	match result {
		Ok(_) => Ok(true),
		Err(_) => Err(PlaylistsError::NotFound)
	}
}

pub fn playlist_by_id(id: &str) -> Result<Playlist, PlaylistsError> {
	let mut client = get_client().unwrap();

	let result: GetResult<Value> = match client
		.get(PLAYLIST_INDEX, id)
		.with_doc_type(PLAYLIST_TYPE)
		.send() {
		Ok(t) => t,
		Err(e) => {
			println!(
				"An error occured fetching playlist with id {:?}: {:?}",
				id,
				e
			);
			return Err(PlaylistsError::NotFound);
		}
	};

	let playlist = Playlist::from_value(&result.source.unwrap(), id.to_string());

	match playlist {
		Some(p) => Ok(p),
		None => Err(PlaylistsError::InvalidPlaylist)
	}
}

pub fn get_playlists() -> Result<Vec<Playlist>, PlaylistsError> {
	let mut client = get_client().unwrap();

	let query = Query::build_match_all().build();

	let result: search::SearchResult<Value> = match client
		.search_query()
		.with_indexes(&[PLAYLIST_INDEX])
		.with_types(&[PLAYLIST_TYPE])
		.with_query(&query)
		.with_size(MAX_RESULTS)
		.send() {
		Ok(result) => result,
		Err(e) => {
			println!("An error occured obtaining search result: {:?}", e);
			return Err(PlaylistsError::SearchError);
		}
	};

	let mut playlists: Vec<Playlist> = Vec::new();

	for result in result.hits.hits {
		let playlist: Option<Playlist> = Playlist::from(&result.source, result.id);

		match playlist {
			Some(p) => {
				playlists.push(p)
			}
			None => {
				continue;
			}
		}

	}

	return Ok(playlists);
}

pub fn create_playlist(playlist: Playlist) -> Result<Playlist, PlaylistsError> {
	let mut client = get_client().unwrap();

	let mut index_op = client.index(PLAYLIST_INDEX, PLAYLIST_TYPE);

	let result = index_op.with_doc(&playlist).send();

	match result {
		Ok(iresult) => {
			let mut result_list = playlist.clone();
			result_list.id = iresult.id;

			return Ok(result_list);
		}
		Err(_) => return Err(PlaylistsError::SaveFailed)
	}
}

fn save_playlist(playlist: &Playlist) -> Result<&Playlist, PlaylistsError> {
	let mut client = get_client().unwrap();

	let mut index_op = client.index(PLAYLIST_INDEX, PLAYLIST_TYPE);

	let result = index_op.with_doc(playlist).with_id(&playlist.id).send();

	match result {
		Ok(_) => Ok(playlist),
		Err(_) => return Err(PlaylistsError::SaveFailed)
	}
}