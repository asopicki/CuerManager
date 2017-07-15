use serde_json::from_str;
use serde_json::Value;

use rocket::testing::MockRequest;
use rocket::http::Method::*;
use rocket::http::Status;

use super::rocket;

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
    let query = "/cuesheets/".to_owned()+id.as_str();

    let mut req = MockRequest::new(Get, query.to_owned());

    let response = req.dispatch_with(&rocket);
    assert_eq!(response.status(), status);

    for header in response.headers() {
        if header.name() == "Content-Type" {
            assert_eq!(header.value(), "text/html; charset=utf-8");
        }
    }
}

fn test_query_cuesheets(query: &str, status: Status) -> Value {
    let rocket = rocket();
    let mut req = MockRequest::new(Get, "/search/".to_owned() + query);

    let mut response = req.dispatch_with(&rocket);
    assert_eq!(response.status(), status);


    let body_data = response.body().and_then(|body| body.into_string()).unwrap_or("[]".to_string());
    let data: Value = from_str(&body_data.to_string()).unwrap();

    assert!(data.as_array().unwrap().len() > 0);

    return data;
}

