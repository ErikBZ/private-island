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
                                .help("The port number to bind the tcp connection to, defaults to some number")
                                .takes_value(true))
                            .arg(Arg::with_name("root")
                                .short("r")
                                .long("root")
                                .value_name("root")
                                .takes_value(true)
                                .required(true))
                            .get_matches();

    // here we check if this stuff is all good
    let ip_addr = match matches.value_of("host"){
        Some(s) => s,
        None => "127.0.0.1"
    };

    let root = match matches.value_of("root"){
        Some(s) => s,
        None => "/home/flipper/Documents/html",
    };

    let cfg = server::Config::from(&root, ip_addr, 8090);

    let http_server = server::Server::from(cfg);
    http_server.listen();
}