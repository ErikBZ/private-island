// file that starts the server
use std::net::{TcpListener, TcpStream};
// because this wasn't in in scope, it would not let the
// stream read bytes form the port
// in wanted str not string

// for the command line
extern crate clap;
use clap::{Arg, App, SubCommand};

// internal crate
mod server;
mod http;

fn main() {
    let matches = App::new("rusty-crab")
                            .version("0.1b")
                            .author("Erik Z. <zapatabrandon@gmail.com>")
                            .about("This is a web server")
                            .arg(Arg::with_name("host")
                                .short("h")
                                .value_name("host_ip")
                                .help("The IP to bind the tcp connection to, defaults to 127.0.0.1")
                                .takes_value(true))
                            .arg(Arg::with_name("port")
                                .short("p")
                                .value_name("port_number")
                                .help("The port number to bind the tcp connection to, defaults to 8888")
                                .takes_value(true))
                            .arg(Arg::with_name("root")
                                .short("r")
                                .long("root")
                                .value_name("root_dir")
                                .help("The root directory of your website project")
                                .takes_value(true)
                                .required(true))
                            .arg(Arg::with_name("logging_type")
                                .long("lt")
                                .value_name("logging type")
                                .possible_values(&["LogFile", "Terminal", "Disabled"])
                                .requires_if("LogFile", "log_file")
                                .help("Defaults to disabled"))
                            .arg(Arg::with_name("log_file")
                                .long("log")
                                .value_name("log file")
                                .help("The file used writing the server logs to"))
                            .get_matches();

    // here we check if this stuff is all good
    let ip_addr = match matches.value_of("host"){
        Some(s) => s,
        None => "127.0.0.1"
    };

    let port_arg = match matches.value_of("port"){
        Some(s) => s,
        None => "8888",
    };
    let port = match port_arg.trim().parse::<u16>(){
        Ok(input_int) => input_int,
        Err(e) => panic!("Input cannot be parsed to unsinged 16 bit number"),
    };

    let root = match matches.value_of("root"){
        Some(s) => s,
        None => "/home/flipper/Documents/html",
    };

    let cfg = server::Config::from(&root, ip_addr, port);

    let http_server = server::Server::from(cfg);
    http_server.listen();
}