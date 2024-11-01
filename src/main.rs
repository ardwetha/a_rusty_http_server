mod response;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::FromStr;

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    println!("New Connection");
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        //Reads the Buffer Line by line
        .lines()
        //Converts from iterator one into iterator two (Unwraps result)
        .map(|result| result.unwrap())
        //Read as long as line is not empty
        .take_while(|line| !line.is_empty())
        //Transform into a vector of type string
        .collect();
    let mut response = response::Response::new();
    response.body = String::from_str("<h1>Hello World</h1>").unwrap();
    for line in http_request {
        println!("{}", line);
    }
    stream.write(response.convert_to_string().as_bytes())?;
    stream.shutdown(Shutdown::Both)
}

fn main() -> std::io::Result<()> {
    println!("Server Starting Up");
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        handle_connection(stream?)?;
    }
    println!("Server shutting down");
    Ok(())
}
