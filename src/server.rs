// this will be my first module
// let's not fuck it up
use std::net::{TcpStream, TcpListener};
use std::time::Duration;
use std::io::Read;
use std::io::Write;
use std::io::prelude::*;
use std::str;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::convert::AsRef;

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
    // root path of project
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
                // maybe i could use from_utf8_lossy to get all the bytes
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

    pub fn load_file(&self, path: &str) -> Result<String, FileLoadError>{
        let mut root_path_buf = PathBuf::from(&self.config.root_path);

        let p: String = String::from(path);
        // i can only match on a String not a &str booo
        match p.as_ref(){
            "\\" => {
                root_path_buf.push("index.html");
            },
            p => {
                root_path_buf.push(p);
            },
        };

        if !root_path_buf.exists(){
            return Err(FileLoadError::PathNotFound)
        }

        // opens the file
        let s = match root_path_buf.to_str(){
            Some(s) => ,
            None => return Err(FileLoadError::PathOutOfBounds),
        };

        let mut contents = String::new();

        Ok(s.to_string())
    }
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

#[allow(dead_code)]
fn allocate_vector(size:usize) -> Vec<u8>{
    let mut new_vec: Vec<u8> = Vec::with_capacity(size);
    for _i in 0..size{
        new_vec.push(0);
    }
    new_vec
}