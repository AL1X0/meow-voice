use anyhow::Result;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting Meow Voice Server (UDP Mode)...");

    // Bind UDP socket
    let socket = UdpSocket::bind("0.0.0.0:4433").await?;
    let socket = Arc::new(socket);
    println!("Listening on 0.0.0.0:4433 (UDP)...");

    // Liste des clients connus
    let clients: Arc<Mutex<Vec<SocketAddr>>> = Arc::new(Mutex::new(Vec::new()));

    let mut buf = [0u8; 2048];

    loop {
        // Recevoir un paquet
        let (len, addr) = match socket.recv_from(&mut buf).await {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Recv error: {:?}", e);
                continue;
            }
        };

        if len == 0 { continue; }

        let data = &buf[..len];
        // println!("Received {} bytes from {}", len, addr);

        // Gestion simpliste des clients (ajout si nouveau)
        let clients_clone = clients.clone();
        let socket_clone = socket.clone();
        let data_vec = data.to_vec();

        tokio::spawn(async move {
            let mut clients_guard = clients_clone.lock().await;
            if !clients_guard.contains(&addr) {
                println!("New client: {}", addr);
                clients_guard.push(addr);
            }

            // Broadcast à tous les autres
            for client_addr in clients_guard.iter() {
                if *client_addr != addr {
                     let _ = socket_clone.send_to(&data_vec, client_addr).await;
                }
            }
        });
    }
}
