use rs_es::Client;
use rs_es::query::Query;
use rs_es::operations::get::GetResult;

use rs_es::operations::search;

use serde_json::Value;

use unescape::unescape;
use comrak::{markdown_to_html, ComrakOptions};
use std::boxed::Box;
use std::ops::Deref;

const MAX_RESULTS: u64 = 200;

const DEFAULT_URL: &'static str = "http://localhost:9200";

const CUESHEET_INDEX: &'static str = "cuesheets";

const CUESHEET_TYPE: &'static str = "cuesheet";

#[derive(Serialize, Deserialize, Debug)]
pub struct CuesheetMetaData {
    id: String,
    title: String,
    phase: String,
    plusfigures: String,
    rhythm: String
}

impl CuesheetMetaData {
    fn new() -> CuesheetMetaData {
        CuesheetMetaData {
            id: String::from(""),
            title: String::from(""),
            phase: String::from(""),
            plusfigures: String::from(""),
            rhythm: String::from("")
        }
    }

    fn from(obj: &Option<Box<Value>>, id: String) -> Option<CuesheetMetaData> {
        match obj {
            &Some(ref v) => {
                let mut cuesheet = CuesheetMetaData::new();
                cuesheet.id = id.clone();

                match v.deref() {
                    &Value::Object(ref obj) => {

                        match obj["metadata"] {
                            Value::Object(ref obj) => {
                                cuesheet.title = String::from(obj.get("title").unwrap_or(&Value::from("")).to_string()).trim_matches('"').to_string();
                                cuesheet.rhythm = String::from(obj.get("rhythm").unwrap_or(&Value::from("")).to_string()).trim_matches('"').to_string();
                                cuesheet.phase = String::from(obj.get("phase").unwrap_or(&Value::from("")).to_string()).trim_matches('"').to_string();
                                cuesheet.plusfigures = String::from(obj.get("plusfigures").unwrap_or(&Value::from("")).to_string()).trim_matches('"').to_string();
                            },
                            _ => return None
                        };

                        Some(cuesheet)
                    },
                    _ => None
                }
            },
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum DocumentsError {
    SearchError,
}

pub fn get_cuesheet(id: &str) -> Result<Box<String>, DocumentsError> {
    let mut client = get_client().unwrap();

    let result: GetResult<Value> = match client
        .get(CUESHEET_INDEX, id)
        .with_doc_type(CUESHEET_TYPE)
        .send() {
        Ok(t) => t,
        Err(e) => {
            println!(
                "An error occured fetching document with id {:?}: {:?}",
                id,
                e
            );
            return Err(DocumentsError::SearchError);
        }
    };

    //println!("Result: {:?}", result);

    let source = result.source.unwrap();
    let content = unescape(source["content"].as_str().unwrap()).unwrap();
    let markdown = markdown_to_html(content.as_str(), &ComrakOptions::default());

    return Ok(Box::new(markdown));

    //return Err(DocumentsError::SearchError)
}

pub fn get_cuesheets_by_phase(phase: &str) -> Result<Vec<CuesheetMetaData>, DocumentsError> {

    let mut client = get_client().unwrap();

    let query = &Query::build_match("phase", phase).build();

    let result: search::SearchResult<Value> = match client
        .search_query()
        .with_indexes(&[CUESHEET_INDEX])
        .with_types(&[CUESHEET_TYPE])
        .with_query(query)
        .with_size(MAX_RESULTS)
        .send() {
        Ok(result) => result,
        Err(e) => {
            println!("An error occured obtaining search result: {:?}", e);
            return Err(DocumentsError::SearchError);
        }
    };

    let mut cuesheets: Vec<CuesheetMetaData> = Vec::new();

    for result in result.hits.hits {
        let cuesheet: Option<CuesheetMetaData> = CuesheetMetaData::from(&result.source, result.id);

        match cuesheet {
            Some(c) => {
                cuesheets.push(c)
            }
            None => {
                continue;
            }
        }

    }

    return Ok(cuesheets);
}

pub fn get_cuesheets(query: &str) -> Result<Vec<CuesheetMetaData>, DocumentsError> {

    let mut client = get_client().unwrap();

    let query = &Query::build_query_string(query)
        .with_default_field("content")
        .with_fields(vec!["title".to_owned(), "rhythm".to_owned()]).build();


    let result: search::SearchResult<Value> = match client
        .search_query()
        .with_indexes(&[CUESHEET_INDEX])
        .with_types(&[CUESHEET_TYPE])
        .with_query(query)
        .with_size(MAX_RESULTS)
        .send() {
        Ok(result) => result,
        Err(e) => {
            println!("An error occured obtaining search result: {:?}", e);
            return Err(DocumentsError::SearchError);
        }
    };

    let mut cuesheets: Vec<CuesheetMetaData> = Vec::new();

    for result in result.hits.hits {
        let cuesheet: Option<CuesheetMetaData> = CuesheetMetaData::from(&result.source, result.id);

        match cuesheet {
            Some(c) => {
                cuesheets.push(c)
            }
            None => {
                continue;
            }
        }

    }

    return Ok(cuesheets);
}

fn get_client() -> Result<Client, DocumentsError> {
    return match Client::new(DEFAULT_URL) {
        Ok(client) => Ok(client),
        Err(e) => {
            println!("An error occured connection to Elasticsearch: {:?}", e);
            return Err(DocumentsError::SearchError);
        }
    };
}
