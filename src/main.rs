// file that starts the server
use std::net::{TcpListener, TcpStream};
// because this wasn't in in scope, it would not let the
// stream read bytes form the port
// in wanted str not string

extern crate socket2;

// internal crate
mod server;
mod http;

fn main() {
    // it looks like somme ports under 1024 require special premission
    let listener = TcpListener::bind("127.0.0.1:8090").unwrap();

    println!("Running socket listener on: {:?}", listener);

    // all of the code has to be in this I guess
    for mut stream in listener.incoming(){
        match stream{
            Ok(mut stream_request) => {
                /*
                server::handle_http_request(&stream_request);
                // when returning result you have to unwrap or match
                // i think write returns the number of bytes written to the stream
                // YAY IT WORKS
                // sending bytes works at least
                // trying to correctly send http responses
                server::send_http_message(&stream_request);
                */

                // lets try something new now
                // ask the server for a file.
                // this will later be changed with the file that is requested by the request
                let file_contents = match server::load_file("/home/flipper/Documents/private-island/src/html/test.html"){
                    Ok(s) => s,
                    Err(e) => panic!("Error tyring to open file: {:?}", e),
                };
                let message = http::HttpMessage::create_simple_http_response(&file_contents);

                // later i will actually check to make sure that message was converted correctly
                server::send_http_response(&stream_request, &message.to_string().unwrap());
            }

            Err(_e) => {
                println!("connection failed")
            }
        }
    }
}