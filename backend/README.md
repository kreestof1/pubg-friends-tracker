# PUBG Friends Tracker - Backend (Rust)

Backend API pour le tracker de joueurs PUBG, développé en Rust avec Axum.

## Prérequis

- Rust 1.75+ ([rustup](https://rustup.rs/))
- MongoDB 7+ (local ou Azure Cosmos DB)
- Clé API PUBG ([obtenir ici](https://developer.pubg.com/))

## Installation

1. Cloner le repository
2. Copier `.env.example` vers `.env` et configurer les variables
3. Installer les dépendances :
```bash
cargo build
```

## Configuration

Modifier le fichier `.env` avec vos valeurs :

```env
MONGODB_URI=mongodb://localhost:27017/pubg-tracker
PUBG_API_KEY=votre_clé_api
CORS_ORIGIN=http://localhost:3000
```

## Lancement

### Mode développement
```bash
cargo run
```

### Mode production
```bash
cargo build --release
./target/release/pubg-tracker-api
```

## Tests

```bash
# Tests unitaires
cargo test

# Tests avec couverture
cargo tarpaulin --out Html

# Linting
cargo clippy -- -D warnings

# Formatage
cargo fmt
```

## Structure du projet

```
src/
├── config/         # Configuration et environnement
├── models/         # Structures de données
├── services/       # Logique métier
├── handlers/       # Handlers HTTP
├── routes/         # Définition des routes
├── middleware/     # Middlewares
├── db/             # Accès base de données
└── utils/          # Utilitaires
```

## API Endpoints

- `GET /health` - Health check
- `GET /ready` - Readiness check
- `POST /api/players` - Ajouter un joueur
- `GET /api/players` - Liste des joueurs
- `GET /api/players/:id` - Détails d'un joueur
- `POST /api/players/:id/refresh` - Rafraîchir les matches
- `GET /api/players/:id/matches` - Matches d'un joueur
- `GET /api/dashboard` - Dashboard comparatif
- `GET /api/players/:id/stats` - Statistiques d'un joueur

## Documentation

La documentation OpenAPI/Swagger sera disponible sur `/api-docs` une fois implémentée.
