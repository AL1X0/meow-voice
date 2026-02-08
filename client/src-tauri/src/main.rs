#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::State;
use tokio::runtime::Runtime;
use tokio::net::UdpSocket;


struct AudioSession {
    running: Arc<AtomicBool>,
}

#[tauri::command]
fn start_audio_session(server_addr: String, state: State<'_, AudioSession>) -> Result<(), String> {
    if state.running.load(Ordering::Relaxed) {
        return Err("Session déjà active".into());
    }
    
    state.running.store(true, Ordering::Relaxed);
    let running = state.running.clone();

    std::thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            match run_audio_loop(server_addr, running.clone()).await {
                 Ok(_) => println!("Session terminée"),
                 Err(e) => eprintln!("Erreur session: {:?}", e),
            }
            running.store(false, Ordering::Relaxed);
        });
    });

    Ok(())
}

#[tauri::command]
fn stop_audio_session(state: State<'_, AudioSession>) -> Result<(), String> {
    state.running.store(false, Ordering::Relaxed);
    Ok(())
}

async fn run_audio_loop(server_addr: String, running: Arc<AtomicBool>) -> anyhow::Result<()> {
    println!("Connecting UDP to {}...", server_addr);
    
    // Bind local port (0 = random)
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect(&server_addr).await?;
    
    println!("Connected! Sending dummy audio...");

    // TODO: Connecter CPAL ici (Code de capture audio réel)
    // Pour ce prototype dockerisé, on simule le flux audio pour être sûr que ça build sans libs sonores complexes
    // Mais CPAL est dans les deps, donc prêt à être décommenté.
    
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(20));
    let mut seq = 0u8;

    while running.load(Ordering::Relaxed) {
        interval.tick().await;
        let payload = vec![seq; 100]; // 100 bytes audio data simulation
        seq = seq.wrapping_add(1);

        if let Err(e) = socket.send(&payload).await {
            eprintln!("Send error: {:?}", e);
            break;
        }
    }

    Ok(())
}

fn main() {
    let session = AudioSession {
        running: Arc::new(AtomicBool::new(false)),
    };

    tauri::Builder::default()
        .manage(session)
        .invoke_handler(tauri::generate_handler![start_audio_session, stop_audio_session])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
