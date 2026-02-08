# Meow Voice - Infrastructure Docker

Ce projet fournit une stack complète Docker pour déployer le serveur Meow Voice, compiler le client pour Windows, et servir le tout via un site web.

## 📂 Structure

- **server/** : Contient le code source du serveur Rust et son `Dockerfile`.
- **client/** : Contient le code source du client Tauri et le `Dockerfile.builder` pour la cross-compilation.
- **website/** : Contient la landing page (`index.html`).
- **docker-compose.yml** : Fichier d'orchestration.

## 🚀 Démarrage Rapide

Pour lancer l'ensemble de l'infrastructure :

```bash
docker compose up -d
```

Cela démarrera :
1.  **meow-server** : Le serveur VoIP sur le port `4433` (UDP/TCP).
2.  **meow-website** : Le site de téléchargement sur le port `80` (accessible via `http://localhost`).

## 🛠️ Compiler le Client Windows

Le service `meow-client-builder` n'est pas lancé par défaut en permanence. Il agit comme un outil de build "à la demande".

Pour générer (ou régénérer) l'installateur Windows `.exe` à partir du code source dans `client/` :

```bash
docker compose up --build meow-client-builder
```

**Ce que cela fait :**
1.  Monte votre dossier `client/` local dans le conteneur.
2.  Compile le projet Tauri pour la cible `x86_64-pc-windows-gnu`.
3.  Génère l'installateur NSIS.
4.  Copie le fichier final `meow-voice-setup.exe` dans le volume partagé `meow-dist`.
5.  Le fichier devient immédiatement téléchargeable sur le site web : `http://localhost/downloads/meow-voice-setup.exe`.

## 📝 Notes Importantes

- **Code Source** : Assurez-vous de placer votre code Rust dans `server/` et votre code Tauri dans `client/`. Les Dockerfiles supposent une structure standard (`Cargo.toml`, `src/`, `package.json`, `src-tauri/`).
- **Volume Partagé** : Le volume `meow-dist` persiste les binaires compilés. Si vous voulez nettoyer, vous pouvez utiliser `docker volume rm meow-voice_meow-dist`.
