use serde_json::from_str;
use serde_json::Value;

use rocket::testing::MockRequest;
use rocket::http::Method::*;
use rocket::http::Status;

use super::rocket;

#[test]
fn test_cuesheet_list() {
   test_query_cuesheets("all", Status::Ok, true);
}

fn test_query_cuesheets(query: &str, status: Status, check_size: bool) {
    let rocket = rocket();
    let mut req = MockRequest::new(Get, "/search/".to_owned() + query);

    let mut response = req.dispatch_with(&rocket);
    assert_eq!(response.status(), status);

    if check_size {
        let body_data = response.body().and_then(|body| body.into_string()).unwrap_or("[]".to_string());

        let data: Value = from_str(&body_data.to_string()).unwrap();


        assert!(data.as_array().unwrap().len() > 0)
    }
}

