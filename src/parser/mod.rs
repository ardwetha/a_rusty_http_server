use crate::response::Response;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::sync::LazyLock;

static EXTENSIONS: LazyLock<HashMap<&str, &str>, fn() -> HashMap<&'static str, &'static str>> =
    LazyLock::new(|| {
        HashMap::from([
            ("html", "text/html"),
            ("js", "text/javascript"),
            ("css", "text/css"),
            ("csv", "text/csv"),
            ("png", "image/png"),
            ("jpeg", "image/jpeg"),
            ("jpg", "image/jpeg"),
            ("gif", "image/gif"),
            ("ico", "image/x-icon"),
        ])
    });

pub fn generate_response(request: Vec<String>) -> (String, Vec<u8>) {
    let mut response = Response::new();

    let head_line = &request[0];

    let split: Vec<String> = head_line.split(&" ").map(|s| s.to_string()).collect();
    match split[0].as_str() {
        "GET" => {
            response = handle_get(request);
        }
        _ => {
            response.status_code = String::from_str("418 I'm a teapot").unwrap();
        }
    }
    (response.convert_to_string(), response.body)
}

fn handle_get(request: Vec<String>) -> Response {
    let mut response = Response::new();
    let line = &request[0];
    let split: Vec<String> = line.split(&" ").map(|s| s.to_string()).collect();

    //ToDo: Search for host header and lookup corresponding data
    let mut requested_resource = split[1].to_owned();

    if requested_resource.contains("..") {
        response.status_code = String::from_str("400 Bad Request").unwrap();
    } else {
        if "/" == requested_resource.as_str() {
            //ToDo: Implement a host file header thing
            requested_resource = String::from_str("index.html").unwrap();
        } else {
            requested_resource = requested_resource[1..].parse().unwrap();
            println!("{}", requested_resource);
        }
        if let Ok(data) = fs::read(&requested_resource) {
            response.content_type = get_content_type(requested_resource.to_owned());
            response.body = data;
        } else {
            response.status_code = String::from_str("404 Not Found").unwrap();
        }
    }
    response
}

fn get_content_type(request_path: String) -> String {
    let index_option = request_path.rfind(&".");
    if let Some(index) = index_option {
        let extension = request_path.split_at(index + 1).1;
        if let Some(content_type) = EXTENSIONS.get(extension) {
            return String::from_str(content_type).unwrap();
        }
    }
    String::from_str("text/html").unwrap()
}
