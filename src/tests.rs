use serde_json::from_str;
use serde_json::Value;

use rocket::http::Status;

use super::rocket;
use rocket::local;

#[test]
fn test_cuesheet_by_id() {
    let documents = test_query_cuesheets("all", Status::Ok);

    let doc = &documents.as_array().unwrap()[0];

    get_cuesheet_by_id(doc["id"].to_string().trim_matches('"').to_string(), Status::Ok);
}

#[test]
fn test_cuesheet_documents() {
    let documents = test_query_cuesheets("all", Status::Ok);

    let doc = &documents.as_array().unwrap()[0];

    assert!(doc["id"].to_string().len() > 0);
}

#[test]
fn test_cuesheet_list() {
   test_query_cuesheets("all", Status::Ok);
}

fn get_cuesheet_by_id(id: String, status: Status) {
    let rocket = rocket();
    let client = local::Client::new(rocket).expect("Rocket setup failed");

    let query = "/cuesheets/".to_owned()+id.as_str();

    let req = client.get(query);

    let response = req.dispatch();
    assert_eq!(response.status(), status);

    let content_type = response.headers().get_one("Content-Type");

    assert_eq!(content_type.is_some(), true);
    assert_eq!(content_type.unwrap(), "text/html; charset=utf-8");
}

fn test_query_cuesheets(query: &str, status: Status) -> Value {
    let rocket = rocket();
    let client = local::Client::new(rocket).expect("Rocket setup failed");

    let req = client.get(format!("/search/{}/",  query));

    let mut response = req.dispatch();
    assert_eq!(response.status(), status);


    let body_data = response.body().and_then(|body| body.into_string()).unwrap_or("[]".to_string());
    let data: Value = from_str(&body_data.to_string()).unwrap();

    assert!(data.as_array().unwrap().len() > 0);

    return data;
}

