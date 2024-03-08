use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        handle_connection(stream); 
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    // If I use the as_str() method in the following line, I get an error.
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    println!("The request line is {:#?}", request_line);

    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => {
            ("HTTP/1.1 200 OK", "hello.html")
        } 
        _ => {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        }
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length =  contents.len();
    let response = format!("{status_line}\r\ncontent len is {length}\r\n\r\n{contents}"); 

    stream.write_all(response.as_bytes()).unwrap();
}