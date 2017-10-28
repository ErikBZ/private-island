// file that starts the server
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream};
// because this wasn't in in scope, it would not let the
// stream read bytes form the port
use std::io::Read;
use std::io::prelude::*;
use std::time::Duration;
extern crate socket2;
use socket2::{Domain, Socket, Type};

fn main() {
    // it looks like somme ports under 1024 require special premission
    let listener = TcpListener::bind("127.0.0.1:8090").unwrap();
    for mut stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                handle_client(stream);
            }
            Err(_e) => {
                println!("connection failed")
            }
        }
    }
}

// looks like stream has to be mutabe in order to actually read things in
fn handle_client(mut stream: TcpStream) -> TcpStream{
    println!("Connection all good");
    let five_seconds = Duration::new(5, 0);
    // for passingn in an option, you wrap the variable with a "Some" and parens not
    // carrots
    stream.set_read_timeout(Some(five_seconds)).unwrap();
    match stream.peer_addr(){
        Ok(addr) =>{
            println!("{:?}, reading from addr", addr);
        }
        Err(_e) => {
            println!("No peer found");
        }
    }

    // trying to send something back here
    let mut buf: Vec<u8> = Vec::with_capacity(1024 as usize);

    match stream.read(&mut buf){
        Ok(size) => {
            println!("Number of bytes read: {:?}", size);
            // & for borrowing
            send_string(&stream);
        }
        Err(_e) =>{
            println!("Oh no something went wrong");
        }
    }

    stream
}

fn send_string(stream: &TcpStream){
    match stream.peer_addr() {
        Ok(addr) => {
            match TcpStream::connect(addr){
                Ok(mut peer) => {
                    println!("Trying to write to {:?}", peer);
                    // I won't be doing anything with this
                    let _ = peer.write("Hello".as_bytes());
                }
                Err(_e) => {
                    println!("Oh no");
                }
            }
        }
        Err(_e) => {
            println!("Something went wrong {:?}", _e)
        }
    }
}