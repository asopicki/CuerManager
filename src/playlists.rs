use std::ops::Deref;
use std::boxed::Box;

use serde_json::Value;

use rs_es::query::Query;
use rs_es::operations::search;
use rs_es::operations::get::GetResult;
use elastic::{PLAYLIST_INDEX, PLAYLIST_TYPE, BackendError, get_client};

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

		//TODO: Add cuesheet refs
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

						for value in values {
							let cuesheetRef = CuesheetRef::new(
								String::from(value.get("id").unwrap_or(&Value::from("")).to_string()).trim_matches('"').to_string(),
								String::from(value.get("title").unwrap_or(&Value::from("")).to_string()).trim_matches('"').to_string(),
							);
							cuesheets.push(cuesheetRef);
						}
						playlist.cuesheets = cuesheets;

						Some(playlist)
					},
					_ => None
				}

			},
			_ => None,
		}
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

	match(playlist) {
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
		Err(e) => return Err(PlaylistsError::SaveFailed)
	}
}