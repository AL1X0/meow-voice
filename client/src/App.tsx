import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
    const [status, setStatus] = useState("Disconnected");
    const [serverIp, setServerIp] = useState("127.0.0.1:4433");

    async function connect() {
        setStatus("Connecting...");
        try {
            // Appel de la fonction Rust 'start_audio_session'
            await invoke("start_audio_session", { serverAddr: serverIp });
            setStatus("Connected & Streaming");
        } catch (e) {
            console.error(e);
            setStatus("Error: " + e);
        }
    }

    async function stop() {
        try {
            await invoke("stop_audio_session");
            setStatus("Stopped");
        } catch (e) {
            setStatus("Error Stop: " + e);
        }
    }

    return (
        <div className="container">
            <h1>Meow Voice Client</h1>
            <p>Status: <strong>{status}</strong></p>

            <div className="row">
                <input
                    id="server-ip"
                    onChange={(e) => setServerIp(e.currentTarget.value)}
                    placeholder="Server IP (e.g. 192.168.1.10:4433)"
                    value={serverIp}
                />
                <button type="button" onClick={connect}>Connect</button>
                <button type="button" onClick={stop}>Stop</button>
            </div>
        </div>
    );
}

export default App;
