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

## Déploiement Docker

L'application peut être déployée dans un conteneur Docker. Pour cela, il suffit de lancer la commande suivante :

# _Docker simple_

```bash
docker build -t rust_api -f Dockerfile .
```

Puis de lancer le conteneur avec la commande suivante :

```bash
docker run -d --rm--name rust_api_container -p 8000:8000 rust_api
```

# _Docker multi-stage_

```bash
docker build -t rust_api_multi -f Dockerfile.multi .
```

Puis de lancer le conteneur avec la commande suivante :

```bash
docker run -d --rm --name rust_api_container -p 8000:8000 rust_api_multi
```

## Vulnerabilités

Apres avoir lancé le contneneur, il est possible de tester les vulnérabilités de l'application.
Pour ce projet j'ai utilisé trivy, un scanner de vulnérabilités pour les images Docker.
Intallation de trivy :

[Lien d'installation](https://trivy.dev/latest/getting-started/installation/)

On scanne l'image du projet avec la commande suivante (dans la console du conteneur trivy) :

```bash
trivy image rust_api
```

Voici le resultat du scan de l'image:

```bash
┌──────────────────────────────────────────────────────────────────────────────────┬────────┬─────────────────┬─────────┐
│                                      Target                                      │  Type  │ Vulnerabilities │ Secrets │
├──────────────────────────────────────────────────────────────────────────────────┼────────┼─────────────────┼─────────┤
│ rust_api (alpine 3.21.3)                                                         │ alpine │        0        │    -    │
├──────────────────────────────────────────────────────────────────────────────────┼────────┼─────────────────┼─────────┤
│ Cargo.lock                                                                       │ cargo  │        0        │    -    │
├──────────────────────────────────────────────────────────────────────────────────┼────────┼─────────────────┼─────────┤
│ usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itoa-1.0.15/Cargo- │ cargo  │        0        │    -    │
│ .lock                                                                            │        │                 │         │
├──────────────────────────────────────────────────────────────────────────────────┼────────┼─────────────────┼─────────┤
│ usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/proc-macro2-1.0.9- │ cargo  │        0        │    -    │
│ 4/Cargo.lock                                                                     │        │                 │         │
├──────────────────────────────────────────────────────────────────────────────────┼────────┼─────────────────┼─────────┤
│ usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/quote-1.0.40/Carg- │ cargo  │        0        │    -    │
│ o.lock                                                                           │        │                 │         │
├──────────────────────────────────────────────────────────────────────────────────┼────────┼─────────────────┼─────────┤
│ usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ryu-1.0.20/Cargo.- │ cargo  │        0        │    -    │
│ lock                                                                             │        │                 │         │
├──────────────────────────────────────────────────────────────────────────────────┼────────┼─────────────────┼─────────┤
│ usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.219/Car- │ cargo  │        0        │    -    │
│ go.lock                                                                          │        │                 │         │
├──────────────────────────────────────────────────────────────────────────────────┼────────┼─────────────────┼─────────┤
│ usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_derive-1.0.- │ cargo  │        0        │    -    │
│ 219/Cargo.lock                                                                   │        │                 │         │
├──────────────────────────────────────────────────────────────────────────────────┼────────┼─────────────────┼─────────┤
│ usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.14- │ cargo  │        0        │    -    │
│ 0/Cargo.lock                                                                     │        │                 │         │
├──────────────────────────────────────────────────────────────────────────────────┼────────┼─────────────────┼─────────┤
│ usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/syn-2.0.100/Cargo- │ cargo  │        0        │    -    │
│ .lock                                                                            │        │                 │         │
└──────────────────────────────────────────────────────────────────────────────────┴────────┴─────────────────┴─────────┘
Legend:
- '-': Not scanned
- '0': Clean (no security findings detected)
```
