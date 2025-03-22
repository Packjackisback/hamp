use crate::transport::{create_socket, connect_socket, send_message, receive_message};
use crate::protocol::Message;
use std::io::{self, Write};
use libc::socket;

pub fn start_client(address: &str, port: u16) {
    let socket = create_socket();
    connect_socket(socket, address, port);
    println!("Connected to the server at {}:{}", address, port);

    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let message = Message::new(input.trim().as_bytes().to_vec());
        send_message(socket, &message).unwrap();

        let response = receive_message(socket).unwrap();
        println!("Server response: {:?}", String::from_utf8_lossy(&response.payload));
    }
}

pub fn start_client_with_message(address: &str, port: u16, input: &str) {
    let socket = create_socket();
    connect_socket(socket, address, port);
    let message = Message::new(input.trim().as_bytes().to_vec());
    send_message(socket, &message).unwrap();
    let response = receive_message(socket).unwrap();
    println!("{:?}", String::from_utf8_lossy(&response.payload));

}