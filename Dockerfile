# Utilisation d'une image Rust
FROM rust:1.78

# Installer cargo-watch
RUN cargo install cargo-watch

RUN cargo install diesel_cli

# Définir le répertoire de travail dans le conteneur
WORKDIR /app

# Copier les fichiers de l'application
COPY . .

# Exposer le port 8080
EXPOSE 8080

# Commande par défaut pour démarrer l'application avec cargo-watch
CMD ["cargo", "watch", "-x", "run"]
