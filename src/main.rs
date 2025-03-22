mod protocol;
mod transport;
mod server;
mod client;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <server|client> [address] [port]", args[0]);
        return;
    }

    let mode = &args[1];
    let address = args.get(2).map_or("127.0.0.1", |s| s.as_str());
    let port = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(7878);
    let message = args.get(4);

    match mode.as_str() {
        "server" => server::start_server(address, port),
        "client" => {
            if(message.is_some()) {
                client::start_client_with_message(address, port, message.unwrap())
            } else {
                client::start_client(address, port)
            }
        },
        _ => eprintln!("Invalid mode: {}", mode),
    }
}