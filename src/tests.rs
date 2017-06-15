use std::fs::File;
use std::io::Read;

use rocket::testing::MockRequest;
use rocket::http::Method::*;
use rocket::http::Status;

use super::rocket;

#[test]
fn test_static_test_file() {
    test_query_file("/static/test.txt", "static/test.txt", Status::Ok, true);
}

#[test]
fn directory_traversal_test() {
    test_query_file(
        "/static/../../../../../../../../../../../../../etc/debian_version",
        "static/../../../../../../../../../../../../../etc_debian_version",
        Status::NotFound,
        false
    );
}

fn test_query_file<T> (path: &str, file: T, status: Status, check_content: bool)
    where T: Into<Option<&'static str>>
{
    let rocket = rocket();
    let mut req = MockRequest::new(Get, &path);

    let mut response = req.dispatch_with(&rocket);
    assert_eq!(response.status(), status);

    if check_content {
        let body_data = response.body().and_then(|body| body.into_bytes());
        if let Some(filename) = file.into() {
            let expected_data = read_file_content(filename);
            assert!(body_data.map_or(false, |s| s == expected_data));
        }
    }
}


fn read_file_content(path: &str) -> Vec<u8> {
    let mut fp = File::open(&path).expect(&format!("Can not open {}", path));
    let mut file_content = vec![];

    fp.read_to_end(&mut file_content).expect(&format!("Reading {} failed.", path));
    file_content
}