// this will be my first module
// let's not fuck it up
use std::net::TcpStream;
use std::time::Duration;
use std::io::Read;
use std::io::Write;
use std::io::prelude::*;
use std::str;
use std::fs::File;

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
            println!("Reading {} bytes into buffer",size);
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
    // when the http header is correct it does not send for some reason
    // telnet works fine
    // curl gets a 52 error, curl seems to work with the 200 status
    // firefox does not work either

    // it looks like we need two new lines after the last header param
    // yupppp
    let response = format!("HTTP/1.0 200 OK\nContent-Type: text/html\nContent-Length: {}\n\n{}", html.len(), html);

    // TODO remove this at some point. for now i'm keeping this for
    // notes
    // for some reason i can't use println here, but i can panic?
    /*
    match File::create("/home/flipper/Documents/private-island/src/resp"){
        Ok(mut f) => f.write_all(response.as_bytes()).unwrap(),
        Err(_) => panic!("couldn't write"),
    };
    */

    // this response works for some reason
    // maybe it has something to do with how I created my reponse?
    // I tried removing parts of the header to see if anything would cause my issue to happen
    // again but it kept working
    /*
    let mut file = match File::open("/home/flipper/Documents/private-island/src/http_response"){
        Ok(f) => f,
        Err(e) => panic!("File was not read correctly. Error {}", e),
    };

    let mut http_response = String::new();
    file.read_to_string(&mut http_response).unwrap();
    */

    println!("\n{}\n", response);
    match stream.write(response.as_bytes()){
        Ok(_) => println!("Response of size {} sent to peer", response.len()),
        Err(_) => println!("Response not sent to peer correctly"),
    }
}

#[allow(dead_code)]
fn allocate_vector(size:usize) -> Vec<u8>{
    let mut new_vec: Vec<u8> = Vec::with_capacity(size);
    for _i in 0..size{
        new_vec.push(0);
    }
    new_vec
}