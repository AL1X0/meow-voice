#!/bin/bash

# Fonction de fallback (Mock Statique avec Pause)
build_mock() {
    echo "⚠️ Building Static Mock Client (Console Pause)..."
    
    # On ajoute une lecture de stdin pour empêcher la fermeture immédiate
    cat <<EOF > mock_client.rs
use std::io;
use std::io::Write;

fn main() {
    println!("===========================================");
    println!("   MEOW VOICE - CLIENT (MOCK VERSION)      ");
    println!("===========================================");
    println!("");
    println!("Si vous voyez ceci, c'est que la compilation");
    println!("de l'interface graphique (Tauri) a échoué");
    println!("dans l'environnement Docker Linux.");
    println!("");
    println!("L'exécutable est fonctionnel mais n'a pas d'UI.");
    println!("");
    println!("Pour obtenir la vraie interface :");
    println!("Compilez le projet 'client/' nativement sur Windows.");
    println!("");
    print!("Appuyez sur ENTRÉE pour quitter...");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
EOF
    
    # Compilation statique
    rustc mock_client.rs --target x86_64-pc-windows-gnu -C target-feature=+crt-static -o mock.exe
    
    mv mock.exe /output/meow-voice-setup.exe
    echo "🎉 Mock Build Complete (Static + Pause)."
}

echo "🚀 Starting Improved Build Process..."

# Nettoyage radical pour éviter les conflits Windows/Linux dans node_modules
# Si node_modules existe (monté depuis Windows), on le supprime ou on essaie de le reconstruire
echo "🧹 Cleaning node_modules..."
rm -rf node_modules package-lock.json

echo "📦 Npm Install (Running in Linux Container)..."
npm install

echo "🔨 Testing Frontend Build (Vite)..."
if npm run build; then
    echo "✅ Frontend Build OK"
    
    echo "🔨 Attempting Tauri Build (Cross-Compile)..."
    # CLI Tauri peut avoir besoin d'arguments pour CI
    if npm run tauri build -- --target x86_64-pc-windows-gnu; then
        echo "✅ Full Tauri Build OK!"
        cp src-tauri/target/x86_64-pc-windows-gnu/release/bundle/nsis/*.exe /output/meow-voice-setup.exe
        exit 0
    else
        echo "❌ Tauri Build Failed (Expected in non-GUI Docker env)."
    fi
else
    echo "❌ Frontend Build Failed."
fi

# Fallback si n'importe quelle étape échoue
build_mock
