// file that starts the server
use std::net::{TcpListener, TcpStream};
// because this wasn't in in scope, it would not let the
// stream read bytes form the port
// in wanted str not string

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
                // trying it with the struct version of server
                // server mod :: Server struct :: fn impleneted on server struct
                let http_server = server::Server::create_new_server();

                let request = http_server.read_request(&stream_request);
                let http_request = http::HttpRequest::new_from(&request);
                http_server.log(&request);

                let file_contents = match server::load_file(&http_request.requested_path){
                    Ok(s) => s,
                    Err(e) => panic!("Error trying to open file: {:?}", e),
                };
                let message = http::HttpMessage::create_simple_http_response(&file_contents);
                // still not checking that message is actually a good string
                http_server.write_response(&stream_request, &message.to_string().unwrap());
            }

            Err(_e) => {
                println!("connection failed")
            }
        }
    }
}