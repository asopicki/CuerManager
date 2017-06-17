

use rs_es::Client;
use rs_es::query::Query;
use rs_es::operations::get::GetResult;

use rs_es::operations::search;

use serde_json::Value;

use unescape::unescape;
use comrak::{markdown_to_html, ComrakOptions};

const MAX_RESULTS: u64 = 200;

const DEFAULT_URL: &'static str = "http://localhost:9200";

const CUESHEET_INDEX: &'static str = "cuesheets";

const CUESHEET_TYPE: &'static str = "cuesheet";

#[derive(Serialize, Deserialize)]
pub struct CuesheetMetaData {
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

pub fn get_cuesheet(id: &str) -> Result<Box<String>, DocumentsError> {
    let mut client = get_client().unwrap();

    let result : GetResult<Value> = match client.get(CUESHEET_INDEX, id).with_doc_type(CUESHEET_TYPE).send() {
        Ok(t) => t,
        Err(e) => {
            println!("An error occured fetching document with id {:?}: {:?}", id, e);
            return Err(DocumentsError::SearchError)
        }
    };

    //println!("Result: {:?}", result);

    let source = result.source.unwrap();
    let content = unescape(source["content"].as_str().unwrap()).unwrap();
    let markdown = markdown_to_html(content.as_str(), &ComrakOptions::default());

    return Ok(Box::new(markdown));

    //return Err(DocumentsError::SearchError)
}

pub fn get_cuesheets(query: &str) -> Result<Vec<CuesheetMetaData>, DocumentsError> {

    let mut client = get_client().unwrap();

    let result : search::SearchResult<Value> = match client.search_query().with_indexes(&[CUESHEET_INDEX])
                    .with_types(&[CUESHEET_TYPE])
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

    let mut cuesheets : Vec<CuesheetMetaData> = Vec::new();

    for value in hits.into_iter() {
        let mut cuesheet: CuesheetMetaData = CuesheetMetaData {
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

fn get_client() -> Result<Client, DocumentsError> {
    return match Client::new(DEFAULT_URL) {
        Ok(client) => Ok(client),
        Err(e) => {
            println!("An error occured connection to Elasticsearch: {:?}", e);
            return Err(DocumentsError::SearchError)
        }
    };
}