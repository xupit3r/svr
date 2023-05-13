use std::{
    fs,
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use  http::{Request, Response, StatusCode}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => println!("{:?}", e)
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("200 OK", "templates/hello.html")
    } else {
        ("404 NOT FOUND", "templates/404.html")
    };


    match fs::read_to_string(filename) {
        Ok(contents) => {
            let response = create_response(status, contents.as_str());

            match stream.write_all(response.as_bytes()) {
                Ok(_) => println!("success response sent"),
                Err(_e) => println!("FUCKING 'LL!")
            }
        },
        Err(_e) => {
            let status = "500 SERVER ERROR";
            let message = "sumfing bad happend 😭";
            let response = create_response(status, message);

            match stream.write_all(response.as_bytes()) {
                Ok(_) => println!("error response sent."),
                Err(_e) => println!("FUCK!")
            }
        }
    }
}

fn parse_request(mut stream: TcpStream) -> Result<Request, io::Error> {
    let buf_reader = BufReader::new(&mut stream);
    let method_uri = buf_reader.lines().next().unwrap();

    match method_uri {
        Ok(preamble) => {
            let headers = buf_reader.lines()
        },
        Err(e) => {
            respond(stream, StatusCode::BAD_REQUEST, "Malformed");
        }
    }
    let request_lines: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
}

fn create_response(status: StatusCode, contents: &str) -> Response<()> {
    Response::buider()
        .status(status)
        .body(contents)
}

fn respond(mut stream: TcpStream, status: &str, contents: &str) {
    let response = create_response(status, contents);
    let bx = response.unwrap();

    match stream.write_all(response.as_bytes()) {
        Ok(_) => println!("response sent."),
        Err(e) => println!("fuck...response not sent {:?}", e)
    }
}
