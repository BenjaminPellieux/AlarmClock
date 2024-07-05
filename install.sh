#!/bin/bash

# Fonction pour vérifier si une commande existe
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Mettre à jour et installer les paquets nécessaires
sudo apt update

# Installer Rust
if ! command_exists rustc; then
    echo "Rust n'est pas installé. Installation de Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "Rust est déjà installé."
fi

# Installer GTK 3
if ! pkg-config --exists gtk+-3.0; then
    echo "GTK 3 n'est pas installé. Installation de GTK 3..."
    sudo apt install -y libgtk-3-dev
else
    echo "GTK 3 est déjà installé."
fi

# Installer yt-dlp
if ! command_exists yt-dlp; then
    echo "yt-dlp n'est pas installé. Installation de yt-dlp..."
    sudo apt install -y python3-pip
    pip3 install yt-dlp
else
    echo "yt-dlp est déjà installé."
fi

# Vérifier l'installation de Firefox
if ! command_exists firefox; then
    echo "Firefox n'est pas installé. Installation de Firefox..."
    sudo apt install -y firefox
else
    echo "Firefox est déjà installé."
fi

# Cloner le projet et construire
if [ ! -d "alarm-clock-app" ]; then
    echo "Clonage du dépôt Git..."
    git clone https://github.com/BenjaminPellieux/AlarmClock.git
fi

cd alarm-clock-app

echo "Construction du projet..."
cargo build -r
cargo run -r

echo "Installation terminée avec succès !"
