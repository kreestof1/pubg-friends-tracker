# PUBG Friends Tracker

Application web de suivi et comparaison de statistiques de joueurs PUBG, avec un dashboard comparatif interactif.

## 🎯 Fonctionnalités

- **Dashboard comparatif** : Comparez plusieurs joueurs avec des visualisations interactives
- **Statistiques détaillées** : K/D, win rate, kills, dégâts, temps de survie, top-1
- **Filtres avancés** : Période (7/30/90 jours), mode de jeu, plateforme
- **Visualisations** : Graphiques en barres, radar chart, leaderboard
- **Gestion des joueurs** : Ajout, recherche, rafraîchissement des données
- **Interface responsive** : Design mobile-first et accessible

## 🏗️ Architecture

- **Backend** : Rust (Axum) avec MongoDB
- **Frontend** : Next.js (React + TypeScript)
- **Déploiement** : Azure (Container Apps + Cosmos DB)
- **API externe** : PUBG Official API

## 📁 Structure du projet

```
pubg-friends-tracker/
├── backend/              # API Rust
│   ├── src/
│   │   ├── config/      # Configuration
│   │   ├── models/      # Modèles de données
│   │   ├── services/    # Logique métier
│   │   ├── handlers/    # Handlers HTTP
│   │   ├── routes/      # Routes API
│   │   ├── middleware/  # Middlewares
│   │   ├── db/          # Base de données
│   │   └── utils/       # Utilitaires
│   └── tests/           # Tests
├── frontend/            # Interface Next.js
│   ├── app/            # Pages (App Router)
│   ├── components/     # Composants React
│   ├── lib/            # Utilitaires et API client
│   └── hooks/          # Custom hooks
├── docs/               # Documentation
└── infra/              # Infrastructure as Code (Bicep)
```

## 🚀 Démarrage rapide

### Prérequis

- Rust 1.75+ ([rustup](https://rustup.rs/))
- Node.js 20+ ([nodejs.org](https://nodejs.org/))
- MongoDB 7+ (local ou Docker)
- Clé API PUBG ([developer.pubg.com](https://developer.pubg.com/))

### Backend

```bash
cd backend
cp .env.example .env
# Configurer les variables dans .env
cargo run
```

Le backend sera accessible sur http://localhost:8080

### Frontend

```bash
cd frontend
cp .env.local.example .env.local
# Configurer NEXT_PUBLIC_API_URL dans .env.local
npm install
npm run dev
```

Le frontend sera accessible sur http://localhost:3000

### Docker Compose (développement local)

```bash
docker-compose up
```

## 🧪 Tests

### Backend
```bash
cd backend
cargo test
cargo clippy
cargo fmt --check
```

### Frontend
```bash
cd frontend
npm test
npm run lint
```

## 📦 Déploiement

Voir le [Plan d'Implémentation](PLAN_IMPLEMENTATION.md) pour les détails complets du déploiement sur Azure.

### Azure Container Apps

1. Provisionner l'infrastructure (Bicep)
2. Build et push des images Docker vers ACR
3. Déploiement via GitHub Actions

## 📚 Documentation

- [Plan d'Implémentation](PLAN_IMPLEMENTATION.md) - Plan détaillé phase par phase
- [Spécifications Fonctionnelles](docs/specifications_fonctionnelles.md)
- [Spécifications Techniques](docs/specifications_techniques.md)
- [Backend README](backend/README.md)
- [Frontend README](frontend/README.md)

## 🔒 Sécurité

- Secrets stockés dans Azure Key Vault
- Managed Identity pour l'accès aux ressources Azure
- CORS configuré en production
- Rate limiting PUBG API (10 req/min) géré avec retry logic
- Pas de secrets en clair dans le code

## 🎨 Stack Technologique

**Backend**
- Rust 1.75+
- Axum (framework web)
- MongoDB / Azure Cosmos DB (API MongoDB)
- Tokio (async runtime)

**Frontend**
- Next.js 14+ (App Router)
- React 18+ avec TypeScript
- Tailwind CSS
- SWR / React Query
- Recharts pour visualisations

**Infrastructure**
- Azure Container Apps
- Azure Cosmos DB (MongoDB API)
- Azure Container Registry
- Azure Key Vault
- Azure Application Insights

## 📊 Métriques et Monitoring

- Application Insights pour logs et métriques
- Dashboard Azure Monitor
- Alertes configurées (erreurs, latence, rate limit)
- Cache hit rate tracking

## 🤝 Contribution

1. Fork le projet
2. Créer une branche feature (`git checkout -b feature/AmazingFeature`)
3. Commit les changements (`git commit -m 'Add AmazingFeature'`)
4. Push vers la branche (`git push origin feature/AmazingFeature`)
5. Ouvrir une Pull Request

## 📝 License

À définir

## 👥 Auteurs

Équipe PUBG Friends Tracker

## 🔗 Liens utiles

- [PUBG API Documentation](https://documentation.pubg.com/)
- [Azure Container Apps](https://learn.microsoft.com/azure/container-apps/)
- [Next.js Documentation](https://nextjs.org/docs)
- [Rust Book](https://doc.rust-lang.org/book/)

---

**Version** : 1.0  
**Date** : 2026-02-09
