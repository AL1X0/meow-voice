# Utiliser l'image officielle Rust
FROM rust:latest

# Définir le répertoire de travail
WORKDIR /app
COPY . .

# Installer les dépendances nécessaires pour WebTransport (QUIC) et autres outils
# protobuf-compiler est souvent requis pour gRPC ou certaines libs Rust
RUN apt-get update && apt-get install -y protobuf-compiler libssl-dev pkg-config libclang-dev build-essential cmake perl && rm -rf /var/lib/apt/lists/*

# NOTE: Cette étape tente de construire le projet si Cargo.toml est présent.
RUN cargo build --release

# Exposition des ports pour WebTransport (UDP est crucial pour QUIC)
EXPOSE 4433/udp
EXPOSE 4433/tcp

# Commande de démarrage par défaut
# Remplacez "meow-server" par le nom de votre binaire généré dans target/release/
CMD ["./target/release/meow-server"]
