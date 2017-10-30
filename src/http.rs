// this will be my first module
// let's not fuck it up
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::io::Read;
use std::io::Write;
use std::str;
// used for formatting strings
use std::fmt;

// since we borrow TcpStream we dont need to return it again to
// pass ownership back
// Reads the request from the tcpStream and returns it
// since our returning string is not contained within TcpStream we cannot
// return a borrowed "&str". We have to create a owner string and pass
// ownership to the caller

// syntax for borrowing a mutable borrow is make the var mut
// and the borrow must be attached to the var type
pub fn handle_http_request(mut stream: &TcpStream) -> String{
    // setting a 3 second timeout duration
    // this won't work with telnet though
    /*
    let dur = Duration::new(3, 0);
    stream.set_read_timeout(Some(dur)).unwrap();
    */

    match stream.peer_addr(){
        Ok(addr) => {
            println!("Connection established with peer: {}", addr);
        },
        Err(_e) => {
            println!("Unable to find peer.")
        },
    }

    // allocating 1024 bytes to read the request
    let mut buffer = allocate_vector(1024 as usize);

    // dont need the assert this time around since i know it works
    // actually just in case
    assert_eq!(buffer.len(), 1024);

    let request = match stream.read(&mut buffer){
        Ok(size) => {
            println!("Reading {} bytes into buffer\n",size);
            // from_utf8 borrows the buffer
            let s = match str::from_utf8(&buffer){
                Ok(string) => string,
                Err(e) => panic!("Buffer read but invalid encoding {}", e),
            };
            s
        }
        Err(e) =>{
            panic!("Error occured while reading into buffer {}", e);
        }
    };

    // converts &str to String
    request.to_string()
}

// before this i'll probably need to parse the http request
// to send the correct response
pub fn send_http_response(mut stream: &TcpStream){
    let html = "<h1>Hello World</h1>";
    let response = format!("HTTP/1.0 200 OK\nContent-Type: text/html\nContent-Length: {}\n{}", html.len(), html);
    stream.write(response.as_bytes()).unwrap();
    println!("Response of size {} sent to peer", response.len());
}

fn allocate_vector(size:usize) -> Vec<u8>{
    let mut new_vec: Vec<u8> = Vec::with_capacity(size);
    for _i in 0..size{
        new_vec.push(0);
    }
    new_vec
}