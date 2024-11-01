use std::io;
use std::io::{BufRead, BufReader};
use std::net::{Shutdown, TcpListener, TcpStream};


fn handle_connection(mut stream: TcpStream) -> io::Result<()>{
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
    for line in http_request {
        println!("{}", line);
    }

    stream.shutdown(Shutdown::Both)
}

fn main() -> std::io::Result<()> {
    println!("Server Starting Up");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

        for stream in listener.incoming() {
            handle_connection(stream?);
        }
    println!("Server shutting down");
    Ok(())
}
