use std::str::FromStr;

mod enum_options;
pub struct Response {
    pub status_code: String,
    pub content_type: String,
    pub connection: enum_options::ConnectionOptions,
    pub body: String,
}

impl Response {
    pub fn new() -> Self {
        //Just some default values
        Response {
            status_code: String::from_str("HTTP/1.1 200 OK").unwrap(),
            content_type: String::from_str("text/html; charset=utf-8").unwrap(),
            connection: enum_options::ConnectionOptions::Close,
            body: String::from_str("").unwrap(),
        }
    }

    //Yes this is basically like a deconstructor, because the function takes ownership
    pub fn convert_to_string(self) -> String {
        let mut response = String::new();
        response.push_str(&self.status_code);
        response.push_str("\r\n");
        response.push_str("Content-Type: ");
        response.push_str(&self.content_type);
        response.push_str("\r\n");
        response.push_str("Server: JustAMessyServer\r\n");
        response.push_str("Connection: ");
        match self.connection {
            enum_options::ConnectionOptions::Close => {
                response.push_str("close\r\n");
            }
            enum_options::ConnectionOptions::KeepAlive => {
                response.push_str("keep-alive\r\n");
            }
        }
        response.push_str("\r\n");
        response.push_str(&self.body);
        response
    }
}
