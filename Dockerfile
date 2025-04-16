# Utilisation de l'image Rust Alpine pour la légèreté
FROM rust:1.85-alpine

# Création d'un utilisateur non-root
RUN adduser -D -u 1001 rustuser

# Définir le répertoire de travail
WORKDIR /

# Copier les fichiers de dépendances
COPY . .

# Installer les dépendances et compiler le projet
RUN cargo fetch && cargo build --release
  
# Utiliser l'utilisateur non-root pour l'exécution
USER rustuser

# Définir le point d'entrée
CMD ["./target/release/rust_api"]
