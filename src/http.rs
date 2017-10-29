// this will be my first module
// let's not fuck it up
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

// does not 
pub fn handle_http_request(mut stream: TcpStream){
    // setting a 3 second timeout duration
    let dur = Duration::new(3, 0);
    stream.set_read_timeout(Some(dur)).unwrap();

    match stream.peer_addr(){
        Ok(addr) => {
            println!("Connection established with peer: {}", addr);
        },
        Err(_e) => {
            println!("Unable to find peer.")
        },
    }

    // allocating 1024 bytes to read the request
    let buffer = allocate_vector(1024 as usize);

}

fn allocate_vector(size:usize) -> Vec<u8>{
    let mut new_vec: Vec<u8> = Vec::with_capacity(size);
    for i in 0..size{
        new_vec.push(0);
    }
    new_vec
}