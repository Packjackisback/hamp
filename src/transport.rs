use libc::{self, c_void, sockaddr, sockaddr_in, socklen_t, AF_INET, SOCK_STREAM, INADDR_ANY};
use std::io::{self, Read, Write};
use std::ptr;
use std::net::{SocketAddrV4, Ipv4Addr};

use crate::protocol::Message;

pub fn create_socket() -> i32 {
    unsafe {
        let socket_fd = libc::socket(AF_INET, SOCK_STREAM, 0);
        if socket_fd < 0 {
            panic!("Failed to create socket");
        }
        socket_fd
    }
}

pub fn bind_socket(socket: i32, address: &str, port: u16) {
    unsafe {
        let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);
        let sockaddr = sockaddr_in {
            sin_family: AF_INET as u16,
            sin_port: addr.port().to_be(),
            sin_addr: libc::in_addr { s_addr: INADDR_ANY },
            sin_zero: [0; 8],
        };
        let sockaddr_ptr = &sockaddr as *const _ as *const sockaddr;

        if libc::bind(socket, sockaddr_ptr, std::mem::size_of::<sockaddr_in>() as u32) < 0 {
            panic!("Failed to bind socket");
        }
    }
}

pub fn listen_socket(socket: i32) {
    unsafe {
        if libc::listen(socket, 128) < 0 {
            panic!("Failed to listen on socket");
        }
    }
}

pub fn accept_connection(socket: i32) -> i32 {
    unsafe {
        let mut addr_len = std::mem::size_of::<sockaddr_in>() as socklen_t;
        let client_socket = libc::accept(socket, ptr::null_mut(), &mut addr_len);
        if client_socket < 0 {
            panic!("Failed to accept connection");
        }
        client_socket
    }
}

pub fn connect_socket(socket: i32, address: &str, port: u16) {
    unsafe {
        let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);
        let sockaddr = sockaddr_in {
            sin_family: AF_INET as u16,
            sin_port: addr.port().to_be(),
            sin_addr: libc::in_addr { s_addr: INADDR_ANY },
            sin_zero: [0; 8],
        };
        let sockaddr_ptr = &sockaddr as *const _ as *const sockaddr;

        if libc::connect(socket, sockaddr_ptr, std::mem::size_of::<sockaddr_in>() as u32) < 0 {
            panic!("Failed to connect to server");
        }
    }
}

pub fn send_message(socket: i32, message: &Message) -> io::Result<()> {
    let encoded = message.encode();
    unsafe {
        if libc::send(socket, encoded.as_ptr() as *const c_void, encoded.len(), 0) < 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to send message"));
        }
    }
    Ok(())
}

pub fn receive_message(socket: i32) -> io::Result<Message> {
    let mut buffer = [0; 512];
    let n = unsafe { libc::recv(socket, buffer.as_mut_ptr() as *mut c_void, buffer.len(), 0) };
    if n < 0 {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to receive message"));
    }
    Message::decode(&buffer[..n as usize]).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Failed to decode message"))
}