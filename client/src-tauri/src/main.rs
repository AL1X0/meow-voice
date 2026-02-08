#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::State;
use tokio::net::UdpSocket;
use tokio::runtime::Runtime;

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

    // Lancer le thread audio/réseau
    std::thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            match run_audio_loop(server_addr, running.clone()).await {
                Ok(_) => println!("Session terminée proprement"),
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

    // 1. Réseau UDP
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect(&server_addr).await?;
    let socket = Arc::new(socket);

    println!("Connected! Starting audio capture...");

    // 2. Audio Capture (CPAL)
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or_else(|| anyhow::anyhow!("No input device"))?;
    
    println!("Input device: {}", device.name()?);

    let config: cpal::StreamConfig = device.default_input_config()?.into();

    // Canal entre Thread Audio (Callback CPAL) et Thread Réseau (Tokio)
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &_| {
            // Conversion f32 -> Bytes (PCM)
            // On envoie des paquets bruts. 
            // Optimisation possible : Opus Codec (mais complexe à ajouter sans statique)
            let mut bytes = Vec::with_capacity(data.len() * 4);
            for &sample in data {
                bytes.extend_from_slice(&sample.to_le_bytes());
            }
            // Envoi des données au thread réseau
            let _ = tx.send(bytes);
        },
        err_fn,
        None, // Timeout
    )?;

    stream.play()?;

    // Boucle d'envoi réseau
    while running.load(Ordering::Relaxed) {
        // On récupère les chunks audio du channel
        if let Some(data) = rx.recv().await {
            // Fragmentation basique si trop gros pour UDP (MTU ~1400)
            // Ici on suppose que CPAL envoie des petits buffers.
            if data.len() < 1200 {
                if let Err(e) = socket.send(&data).await {
                    eprintln!("UDP Send Error: {:?}", e);
                    break;
                }
            } else {
                // Si trop gros, on drop ou on split (pour l'instant on drop pour simplifier)
                // eprintln!("Packet too big: {}", data.len());
            }
        }
    }

    Ok(()) // Le stream audio est droppé ici et s'arrête
}

fn main() {
    let session = AudioSession {
        running: Arc::new(AtomicBool::new(false)),
    };

    tauri::Builder::default()
        .manage(session)
        .invoke_handler(tauri::generate_handler![
            start_audio_session,
            stop_audio_session
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
