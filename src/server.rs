use crate::transport::{create_socket, bind_socket, listen_socket, accept_connection, receive_message, send_message};
use crate::protocol::Message;

pub fn start_server(address: &str, port: u16) {
    let socket = create_socket();
    bind_socket(socket, address, port);
    listen_socket(socket);
    println!("Server listening on {}:{}", address, port);

    loop {
        let client_socket = accept_connection(socket);
        std::thread::spawn(move || {
            let message = receive_message(client_socket).unwrap();
            println!("Received: {:?}", String::from_utf8_lossy(&message.payload));

            let response = Message::new(b"uwu :3".to_vec());
            send_message(client_socket, &response).unwrap();
            unsafe { libc::close(client_socket) };
        });
    }
}