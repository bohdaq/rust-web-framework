//! # rust-web-server
//!
//! `rust-web-server` provides the collection of utility functions and modules used to build Rust Web Server. Can be useful while developing HTTP related functionality.
//!

pub mod app;
pub mod body;
pub mod client_hint;
pub mod cors;
pub mod entry_point;
pub mod ext;
pub mod header;
pub mod http;
pub mod language;
pub mod mime_type;
pub mod range;
pub mod request;
pub mod response;
pub mod server;
pub mod symbol;
pub mod thread_pool;
pub mod log;
pub mod core;
pub mod null;
pub mod json;


use crate::entry_point::{bootstrap, get_ip_port_thread_count, set_default_values};
use crate::server::Server;
use crate::thread_pool::ThreadPool;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use crate::log::Log;
use crate::symbol::SYMBOL;

pub fn start_server() {

    let info = Log::info("Rust Web Server");
    println!("{}", info);
    let usage = Log::usage_information();
    println!("{}", usage);


    println!("RWS Configuration Start: \n");
    set_default_values();
    bootstrap();
    println!("RWS Configuration End\n\n");


    let (ip, port, thread_count) = get_ip_port_thread_count();
    create_tcp_listener_with_thread_pool(ip.as_str(), port, thread_count);
}

pub fn create_tcp_listener_with_thread_pool(ip: &str, port: i32, thread_count: i32) {
    let bind_addr = [ip, SYMBOL.colon, port.to_string().as_str()].join(SYMBOL.empty_string);
    println!("Setting up server at protocol: http, ip {}, port {}", ip, port);

    let boxed_listener = TcpListener::bind(&bind_addr);
    if boxed_listener.is_err() {
        eprintln!("unable to set up TCP listener: {}", boxed_listener.err().unwrap());
    } else {
        let listener = boxed_listener.unwrap();
        let pool = ThreadPool::new(thread_count as usize);

        let server_url_thread_count = Log::server_url_thread_count("http", &bind_addr, thread_count);
        println!("{}", server_url_thread_count);

        for boxed_stream in listener.incoming() {
            if boxed_stream.is_err() {
                eprintln!("unable to get TCP stream: {}", boxed_stream.err().unwrap());
            } else {
                let stream = boxed_stream.unwrap();

                pool.execute(move || {

                    let mut peer_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0)), 0);
                    let boxed_peer_addr = stream.peer_addr();
                    if boxed_peer_addr.is_ok() {
                        peer_addr = boxed_peer_addr.unwrap()
                    } else {
                        eprintln!("\nunable to read peer addr");
                    }

                    Server::process_request(stream, peer_addr);
                });
            }
        }
    }

}
