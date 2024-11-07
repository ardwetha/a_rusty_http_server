extern crate core;

mod parser;
mod response;

use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    loop {
        println!("New Connection");
        let mut buf_reader = BufReader::new(&stream);
        let mut buf: Vec<u8> = Vec::new();
        let mut http_request: Vec<String> = Vec::new();

        //Taking care of the \r elsewhere
        //Reading line by line
        while buf_reader.read_until(b'\n', &mut buf).is_ok() {
            let data = String::from_utf8_lossy(&buf); // More efficient way to handle bytes

            // Trim the \r and \n characters
            let line = data.trim().to_string();
            // Check for an empty line (end of headers)

            if line.is_empty() {
                break; // End of request headers
            }

            // Add the line to the request
            http_request.push(line);

            // Clear the buffer for the next read
            buf.clear();
        }

        if http_request.len() == 0 {
            stream.shutdown(Shutdown::Both)?;
            println!("Conenction closed");
            break;
        }
        for line in http_request.to_owned() {
            println!("{} END", line);
        }
        let (header, body, keep_alive) = parser::generate_response(http_request);
        stream.write_all(header.as_bytes())?;
        stream.write_all(body.as_slice())?;
        stream.flush()?;
        println!("{}", header);
        if !keep_alive {
            stream.shutdown(Shutdown::Both)?;
            println!("Connection closed");
            break;
        }
        stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("Server Starting Up");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream_res in listener.incoming() {
        if let Ok(stream) = stream_res {
            thread::spawn(move || handle_connection(stream));
        }
    }
    println!("Server shutting down");
    Ok(())
}
