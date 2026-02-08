# Utiliser une image de base avec Node.js (nécessaire pour le CLI Tauri)
FROM node:20-bookworm

# Installer Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Installer les dépendances système pour Tauri et la cross-compilation Windows
# gcc-mingw-w64-x86-64 et nsis sont CRUCIAUX pour générer l'installateur .exe
RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    gcc-mingw-w64-x86-64 \
    nsis \
    pkg-config \
    libasound2-dev \
    && rm -rf /var/lib/apt/lists/*

# Ajouter la cible de compilation Rust pour Windows 64-bit
RUN rustup target add x86_64-pc-windows-gnu

# Définir le répertoire de travail
WORKDIR /app

# Point d'entrée pour la construction
COPY build_client.sh /usr/local/bin/build_client.sh
RUN chmod +x /usr/local/bin/build_client.sh

CMD ["/usr/local/bin/build_client.sh"]
