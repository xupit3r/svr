use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

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
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };


    match fs::read_to_string(filename) {
        Ok(contents) => {
            let length = contents.len();
            let response =
                format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            match stream.write_all(response.as_bytes()) {
                Ok(_o) => println!("success response sent"),
                Err(_e) => println!("FUCKING 'LL!")
            }
        },
        Err(_e) => {
            let message = "sumfing bad happend 😭";
            let length = message.len();
            let response = 
                format!("HTTP/1.1 500 SERVER ERROR\r\nContent-Length: {length}\r\n\r\n{message}");

            match stream.write_all(response.as_bytes()) {
                Ok(_o) => println!("error response sent."),
                Err(_e) => println!("FUCK!")
            }
        }
    }
}
