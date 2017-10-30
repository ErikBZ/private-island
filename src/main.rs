// file that starts the server
use std::net::{TcpListener, TcpStream};
// because this wasn't in in scope, it would not let the
// stream read bytes form the port
use std::io::Read;
use std::io::prelude::*;
use std::time::Duration;
// in wanted str not string
use std::str;

extern crate socket2;

// internal crate
mod http;

fn main() {
    // it looks like somme ports under 1024 require special premission
    let listener = TcpListener::bind("127.0.0.1:8090").unwrap();

    // all of the code has to be in this I guess
    for mut stream in listener.incoming(){
        match stream{
            Ok(mut stream_request) => {
                http::handle_http_request(&stream_request);
                // when returning result you have to unwrap or match
                // i think write returns the number of bytes written to the stream
                // YAY IT WORKS
                // sending bytes works at least
                // trying to correctly send http responses
                http::send_http_response(&stream_request);
            }
            Err(_e) => {
                println!("connection failed")
            }
        }
    }
}

fn alloc_vector(size: usize) -> Vec<u8>{
    let mut vec : Vec<u8> = Vec::with_capacity(size);
    for _i in 0..size{
        vec.push(0);
    }
    vec
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

fn print_response(buf: &Vec<u8>){
    let s = match str::from_utf8(buf){
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-9 sequence: {}", e),
    }; // remember to put semicolon at the end of this block

    println!("Message:\n {}\n", s);
}
