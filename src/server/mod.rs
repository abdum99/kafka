mod handlers;

use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::{
    common::{
        request::{self, KafRequest},
        response::KafResponse,
        DecodeFromBytes, EncodeToBytes,
    }, server::handlers::handle_request, StrError
};

pub fn handle_stream(mut stream: TcpStream) -> Result<(), std::io::Error> {
    // Read the 4-byte message length prefix
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf)?;
    let message_len = i32::from_be_bytes(len_buf) as usize;

    println!("Message length: {}", message_len);

    // Read exactly message_len bytes
    let mut buf = vec![0u8; message_len];
    stream.read_exact(&mut buf)?;

    println!("buf: {:x?}{:x?}", message_len.to_be_bytes(), buf);

    let mut offset = 0;
    let request = KafRequest::read_from_u8(&buf, &mut offset).expect("failed to read request");
    println!("received request: {:#?}", request);

    // CALL: handle_request
    let response = handle_request(request).expect("failed to get a response");
    let response_bytes = response.write_to_bytes();

    println!("writing response: ${:x?}", response_bytes);
    stream.write_all(&response_bytes)?;

    Ok(())
}
