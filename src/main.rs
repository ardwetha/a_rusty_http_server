extern crate core;

mod parser;
mod response;

use std::io::{self, Write};
use std::io::{BufRead, BufReader};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::time::Duration;

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    loop {
        println!("New Connection");
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<String> = buf_reader
            //Reads the Buffer Line by line
            .lines()
            //Converts from iterator one into iterator two (Unwraps result)
            .map(|result| result.unwrap())
            //Read as long as line is not empty
            .take_while(|line| !line.is_empty())
            //Transform into a vector of type string
            .collect();

        if http_request.len() == 0 {
            stream.shutdown(Shutdown::Both)?;
            break;
        }
        for line in http_request.to_owned() {
            println!("{} END", line);
        }
        let (header, body, keep_alive) = parser::generate_response(http_request);
        stream.write(header.as_bytes())?;
        stream.write(body.as_slice())?;
        if !keep_alive {
            stream.shutdown(Shutdown::Both)?;
            break;
        }
        stream.set_read_timeout(Some(Duration::from_secs_f64(5.0)))?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("Server Starting Up");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        handle_connection(stream?)?;
    }
    println!("Server shutting down");
    Ok(())
}
