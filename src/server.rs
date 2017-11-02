// this will be my first module
// let's not fuck it up
use std::net::{TcpStream, TcpListener};
use std::time::Duration;
use std::io::Read;
use std::io::Write;
use std::io::prelude::*;
use std::str;
use std::fs::File;

// enum for diffferent reasons that an html load may take place
#[derive(Debug)]
pub enum FileLoadError {
    PathNotFound,
    PathOutOfBounds,
}

#[derive(Debug)]
// log to file, terminal or none?
// maybe that will be the best course
// where to write the logs
enum LoggingType {
    LogFile{path: String},
    Terminal,
    Disabled,
}

// how verbose
// i have to figure out what my "levels" are
enum LoggingLevel{
}

struct Config{
    root_path: String,
    logging: LoggingType,
}

// we create a server, and then run it
// the server can load a file, handle a request, send a response
// to load a file it double checks the path
pub struct Server{
    // tells the server how to act
    config: Config,
    // listens for connections
    // not using this yet
    //listener: TcpListener,
}

impl Server{
    pub fn create_new_server() -> Server{
        Server{
            config: Config{
                root_path: String::from("/home/flipper/Documents/private-island/src/html"),
                logging:  LoggingType::Terminal,
            }
        }
    }

    // we won't use this for now
    #[allow(dead_code)]
    pub fn listen(self){
        /*
        for mut stream in self.listener.incoming(){
            match stream{
                Ok(_s) => {
                    // do stuff
                }
                Err(e) => println!("Error trying to connect to some source: {:?}", e),
            }
        }
        */
    }

    // reads the bytes from the stream and returns it as a string
    // should this check if it's an http request or should another
    // function do that??
    // it should be &self cause you have to borrow yourself
    pub fn read_request(&self, mut stream: &TcpStream) -> String{
        let mut buffer = allocate_vector(1024 as usize);
        let request = match stream.read(&mut buffer){
            Ok(_size) => {
                self.log("Read message");
                let request = match str::from_utf8(&buffer){
                    Ok(s) => s,
                    Err(e) => panic!("Improper message, cannot convert to string. {:?}", e),
                };
                request
            },
            // i should figure out a better way to recover from this
            Err(_e) => panic!("Could not read from buffer"),
        };
        request.to_string()
    }

    // writes the response to the stream which sends it to the peer
    pub fn write_response(&self, mut stream: &TcpStream, message: &str){
        match stream.write(message.as_bytes()){
            Ok(_s) => self.log("Message sent"),
            Err(_e) => println!("Could not send message to peer"),
        } 
    }
    
    // if logging is enabled this 
    pub fn log(&self, message: &str){
        let cfg = &self.config.logging;

        match self.config.logging{
            // this was complaining about the movment of path from the enum
            // to the the variable path here. I have to pass a reference to path
            // using the keyword
            LoggingType::LogFile{ref path} => {
                println!("This will log any thing into {}", path);
            },
            LoggingType::Terminal => {
                println!("{}", message);
            },
            // do nothing here
            LoggingType::Disabled => (),
        }
    }
}
// for now i'm keeping this stuff, but i will be deleting this at some point

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

// for now this doesn't check for files outside the root dir
pub fn load_file(path: &str) -> Result<String, FileLoadError>{
    let mut file = match File::open(path){
        Ok(mut f) => f,
        Err(_) => return Err(FileLoadError::PathNotFound),
    };

    // saving contents here
    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents){
        Ok(_) => {},
        Err(e) => panic!("Error reading file contents to string: {}", e),
    }
    Ok(file_contents)
}

pub fn send_http_response(mut stream: &TcpStream, message: &str){
   match stream.write(message.as_bytes()){
       Ok(_s) => println!("Message sent"),
       Err(e) => panic!("Error to send message: {}", e),
   }
}

// before this i'll probably need to parse the http request
// to send the correct response
pub fn send_http_message(mut stream: &TcpStream){
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