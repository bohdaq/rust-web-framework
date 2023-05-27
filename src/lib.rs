//! # rust-web-server
//!
//! `rust-web-server` provides the collection of utility functions and modules used to build Rust Web Server. Can be useful while developing HTTP related functionality.
//!

pub mod app;
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
pub mod body;
pub mod json;
pub mod null;
pub mod url;
pub mod core;
pub mod application;
pub mod controller;


use crate::app::App;
use crate::server::Server;
use crate::application::Application;
use crate::core::New;

pub fn start(app: impl Application + New + Send + 'static + Copy) {

    let new_server = Server::setup();
    if new_server.is_err() {
        eprintln!("{}", new_server.as_ref().err().unwrap());
    }


    let (listener, pool) = new_server.unwrap();

    Server::run(listener, pool, app);
}

pub fn start_server() {

    let new_server = Server::setup();
    if new_server.is_err() {
        eprintln!("{}", new_server.as_ref().err().unwrap());
    }


    let (listener, pool) = new_server.unwrap();
    let app = App::new();


    Server::run(listener, pool, app);
}

