## Installation des dépendances

L'application nécessite d'avoir installé Rust et Cargo pour fonctionner.
Elle utilise également la "crate" `serde` & `serde_json` pour serialiser en JSON les headers d'une requète lors d'un `GET /ping`.

Pour installer les dépendances, il suffit de lancer la commande suivante :

```bash
cargo add serde serde_json
```

## Lancement du serveur Rust

Pour lancer l'application, il suffit de lancer la commande suivante :

```bash
cargo run
```

## Variables d'environnement

L'application écoute une variable d'env: **PING_LISTEN_PORT**

La variable n'est pas nécéssaire au bon fonctionnement de l'application, mais elle permet de changer le port sur lequel le serveur écoute. Par défaut, le serveur écoute sur le port 8000.

Pour la changer il suffit de lancer la commande suivante :

_Powershell_

```powershell
$Env:PING_LISTEN_PORT=7989; cargo run
```

_Bash_

```bash
PING_LISTEN_PORT=7989 cargo run
```
