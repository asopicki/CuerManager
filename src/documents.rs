

use rs_es::Client;
use rs_es::query::Query;

use rs_es::operations::search;

use serde_json::Value;

const MAX_RESULTS: u64 = 200;

const DEFAULT_URL: &'static str = "http://localhost:9200";


#[derive(Serialize, Deserialize)]
pub struct CuesheetDocument {
    id: String,
    title: Option<String>,
    phase: Option<String>,
    plusfigures: Option<String>,
    rhythm: Option<String>,
    version: Option<u64>
}


#[derive(Debug)]
pub enum DocumentsError {
    SearchError
}

pub fn get_cuesheets(query: &str) -> Result<Vec<CuesheetDocument>, DocumentsError> {

    //TODO: Use constants instead of hard coded values
    let mut client = match Client::new(DEFAULT_URL) {
        Ok(client) => client,
        Err(e) => {
            println!("An error occured connection to Elasticsearch: {:?}", e);
            return Err(DocumentsError::SearchError)
        }
    };


    let result:search::SearchResult<Value> = match client.search_query().with_indexes(&["cuesheets"])
                    .with_types(&["cuesheet"])
                    .with_query(&Query::build_match_all().build())
                    .with_size(MAX_RESULTS)
                    .send() {
        Ok(result) => result,
        Err(e) => {
            println!("An error occured obtaining search result: {:?}", e);
            return Err(DocumentsError::SearchError)
        }
    };

    let hits = result.hits.hits;

    let mut cuesheets : Vec<CuesheetDocument> = Vec::new();

    for value in hits.into_iter() {
        let mut cuesheet:CuesheetDocument = CuesheetDocument{
            id: "-1".to_string(),
            title: None,
            phase: None,
            plusfigures: None,
            rhythm: None,
            version: None
        };

        //println!("{:?}", value);

        cuesheet.id = value.id;
        cuesheet.version = value.version;

        let metadata = match value.source {
            Some(obj) => Some(obj["metadata"].clone()),
            None => None
        };

        if metadata.is_some() {
            let data = metadata.unwrap();
            cuesheet.title = Some(String::from(data["title"].to_string()).trim_matches('"').to_string());
            cuesheet.rhythm = Some(String::from(data["rhythm"].to_string()).trim_matches('"').to_string());
            cuesheet.phase = Some(String::from(data["phase"].to_string()).trim_matches('"').to_string());
            cuesheet.plusfigures = Some(String::from(data["plusfigures"].to_string()).trim_matches('"').to_string());
        }

        cuesheets.push(cuesheet);
    }

    return Ok(cuesheets);
}