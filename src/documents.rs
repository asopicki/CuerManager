use rs_es::query::Query;
use rs_es::query::full_text;
use rs_es::operations::get::GetResult;

use rs_es::operations::search;

use serde_json::Value;

use unescape::unescape;
use comrak::{markdown_to_html, ComrakOptions};
use std::boxed::Box;
use std::ops::Deref;

use elastic::{CUESHEET_INDEX, CUESHEET_TYPE, BackendError, get_client};

const MAX_RESULTS: u64 = 200;

#[derive(Serialize, Deserialize, Debug)]
pub struct CuesheetMetaData {
    id: String,
    title: String,
    phase: String,
    plusfigures: String,
    rhythm: String,
    score: f64
}

impl CuesheetMetaData {
    fn new() -> CuesheetMetaData {
        CuesheetMetaData {
            id: String::from(""),
            title: String::from(""),
            phase: String::from(""),
            plusfigures: String::from(""),
            rhythm: String::from(""),
            score: 0.0
        }
    }

    fn from(obj: &Option<Box<Value>>, id: String, score: Option<f64>) -> Option<CuesheetMetaData> {
        match obj {
            &Some(ref v) => {
                let mut cuesheet = CuesheetMetaData::new();
                cuesheet.id = id.clone();
                cuesheet.score = score.unwrap_or(0.0);

                match v.deref() {
                    &Value::Object(ref obj) => {

                        cuesheet.title = String::from(obj.get("title").unwrap_or(&Value::from("")).to_string()).trim_matches('"').to_string();
                        cuesheet.phase = String::from(obj.get("phase").unwrap_or(&Value::from("")).to_string()).trim_matches('"').to_string();
                        cuesheet.plusfigures= String::from(obj.get("plusfigures").unwrap_or(&Value::from("")).to_string()).trim_matches('"').to_string();
                        cuesheet.rhythm = String::from(obj.get("rhythm").unwrap_or(&Value::from("")).to_string()).trim_matches('"').to_string();

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

#[derive(Debug)]
pub enum SearchType {
    PhaseSearch,
    RhythmSearch,
    StringSearch
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

    let source = result.source.unwrap();
    let content = unescape(source["content"].as_str().unwrap()).unwrap();
    let markdown = markdown_to_html(content.as_str(), &ComrakOptions::default());

    return Ok(Box::new(markdown));
}

pub fn get_cuesheets(query: &str) -> Result<Vec<CuesheetMetaData>, DocumentsError> {

    let has_search_prefix = query.contains(":");

    if has_search_prefix {
        let parts = query.splitn(2, ":").collect::<Vec<_>>();
        let mut iter = parts.iter();
        let (search_type, search_string) = (iter.next().unwrap(), iter.next().unwrap());

        let search_type = match *search_type {
            "phase" => SearchType::PhaseSearch,
            "rhythm" => SearchType::RhythmSearch,
            _ => SearchType::StringSearch
        };

        let search_query = match search_type {
            SearchType::PhaseSearch => build_match_query("phase", search_string),
            SearchType::RhythmSearch => build_match_query("rhythm", search_string),
            SearchType::StringSearch => build_default_query(query)
        };

        return run_search(&search_query);
    } else {
        return run_search(&build_default_query(&query));
    }


}

fn run_search(query: &Query) -> Result<Vec<CuesheetMetaData>, DocumentsError>  {
    let mut client = get_client().unwrap();

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
        let cuesheet: Option<CuesheetMetaData> = CuesheetMetaData::from(&result.source, result.id, result.score);

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

fn build_match_query(term: &str, query: &str) -> Query {
    Query::build_match(term, query).with_operator("and").build()
}

fn build_default_query(query: &str) -> Query {
    Query::build_match("content", query).with_type(full_text::MatchType::PhrasePrefix)
        .with_slop(10).build()
}

