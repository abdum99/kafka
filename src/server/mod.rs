use std::{io::Write, net::TcpStream};

use crate::common::response::Response;

fn get_response() -> Result<Response, String> {
    Ok(Response::new(
        4,
        7,
    ))
}

pub fn handle_request(mut stream: TcpStream) -> Result<(), std::io::Error> {
    let response_vec = get_response().expect("failed to get a response").to_vec();
    let res = stream.write_all(&response_vec);
    println!("wrote response: ${:#?}", response_vec);

    res
}

