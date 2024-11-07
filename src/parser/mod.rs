use crate::config::Config;
use crate::response::{self, Response};
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

pub fn generate_response(request: Vec<String>, conf: &Config) -> (String, Vec<u8>, bool) {
    let mut response = Response::new();
    let head_line = &request[0];

    let split: Vec<String> = head_line.split(&" ").map(|s| s.to_string()).collect();
    match split[0].as_str() {
        "GET" => {
            (response) = handle_get(request, &conf);
        }
        _ => {
            response.status_code = String::from_str("418 I'm a teapot").unwrap();
        }
    }
    let keep_alive = match response.connection {
        response::enum_options::ConnectionOptions::Close => false,
        response::enum_options::ConnectionOptions::KeepAlive => true,
    };
    (response.convert_to_string(), response.body, keep_alive)
}

fn handle_get(request: Vec<String>, conf: &Config) -> Response {
    let mut response = Response::new();
    let line = &request[0];
    let split: Vec<String> = line.split(&" ").map(|s| s.to_string()).collect();

    //ToDo: Search for host header and lookup corresponding data
    let mut requested_resource = split[1].to_owned();

    let mut full_path = conf.file_root.to_owned();
    //Why not working -> ToDo: Fix
    if requested_resource.contains("..") {
        response.status_code = String::from_str("400 Bad Request").unwrap();
        return response;
    } else {
        let mut host = String::new();
        for str in request.iter().to_owned() {
            if str.contains("Host: ") {
                let splitted: Vec<String> = str.split(" ").map(|s| s.to_string()).collect();
                host = splitted[1].to_owned();

                //Necessary to prevent directory traversal attacks via a maliciously crafted host name
                if host.contains("/") {
                    host = String::from_str("").unwrap();
                }
            }
            if str.contains("Connection:") {
                let splitted: Vec<String> = str.split(" ").map(|s| s.to_string()).collect();
                let param = splitted[1].to_owned();
                if (param.trim() == "keep-alive" || param.trim() == "Keep-Alive")
                    && conf.keep_alive == true
                {
                    response.connection = response::enum_options::ConnectionOptions::KeepAlive;
                }
            }
        }
        if host.is_empty() {
            host = conf.default_host.to_owned();
        }
        full_path.push_str(&host[0..]);
        full_path.push_str("/");
        if "/" == requested_resource.as_str() {
            requested_resource = String::from_str("index.html").unwrap();
        } else {
            requested_resource = requested_resource[1..].parse().unwrap();
            println!("{}", requested_resource);
        }
        full_path.push_str(&requested_resource[0..]);
        println!("FullPath: {}", full_path);
        if let Ok(data) = fs::read(&full_path) {
            response.content_type = get_content_type(requested_resource.to_owned());
            response.body = data;
        } else {
            response.status_code = String::from_str("404 Not Found").unwrap();
            response.connection = response::enum_options::ConnectionOptions::Close;
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
