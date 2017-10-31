// file that starts the server
use std::net::{TcpListener, TcpStream};
// because this wasn't in in scope, it would not let the
// stream read bytes form the port
// in wanted str not string

extern crate socket2;

// internal crate
mod server;

fn main() {
    // it looks like somme ports under 1024 require special premission
    let listener = TcpListener::bind("127.0.0.1:8090").unwrap();

    println!("Running socket listener on: {:?}", listener);

    // all of the code has to be in this I guess
    for mut stream in listener.incoming(){
        match stream{
            Ok(mut stream_request) => {
                server::handle_http_request(&stream_request);
                // when returning result you have to unwrap or match
                // i think write returns the number of bytes written to the stream
                // YAY IT WORKS
                // sending bytes works at least
                // trying to correctly send http responses
                server::send_http_response(&stream_request);
            }
            Err(_e) => {
                println!("connection failed")
            }
        }
    }
}