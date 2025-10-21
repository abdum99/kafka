#![allow(unused_imports)]
use std::net::TcpListener;

use crate::server::handle_stream;

pub mod common;
pub mod utils;
pub mod server;

pub type StrError = String;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                let _ = handle_stream(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
