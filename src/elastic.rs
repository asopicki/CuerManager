use rs_es::Client;

const DEFAULT_URL: &'static str = "http://localhost:9200";

pub const CUESHEET_INDEX: &'static str = "cuesheets";

pub const CUESHEET_TYPE: &'static str = "cuesheet";

pub const PLAYLIST_INDEX: &'static str = "playlists";

pub const PLAYLIST_TYPE: &'static str = "playlist";

#[derive(Debug)]
pub enum BackendError {
	ClientError
}


pub fn get_client() -> Result<Client, BackendError> {
	return match Client::new(DEFAULT_URL) {
		Ok(client) => Ok(client),
		Err(e) => {
			println!("An error occured connection to Elasticsearch: {:?}", e);
			return Err(BackendError::ClientError);
		}
	};
}