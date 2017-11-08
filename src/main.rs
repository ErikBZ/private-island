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

                match http_server.load_file(&http_request.requested_path){
                    Ok(s) => {
                        // forgetting to add :: after http throws a "not a type expecting type here
                        // because of type ascrption" compile error
                        let message = http::HttpMessage::create_simple_http_response(&s);
                        http_server.write_response(&stream_request, &message.to_string().unwrap());
                    },
                    Err(e) => {
                        println!("Error trying to open file: {:?}", e);
                        println!("Skipping this peer");
                    },
                };
            }

            Err(_e) => {
                println!("connection failed")
            }
        }
    }
}