# Plan d'Implémentation - PUBG Friends Tracker

## Vue d'ensemble
Application de suivi des joueurs et matches PUBG avec déploiement sur Azure.
- **Backend** : Rust (API REST)
- **Frontend** : Next.js (React + TypeScript)
- **Fonctionnalité principale** : Dashboard comparatif ludique permettant de comparer les statistiques de plusieurs joueurs PUBG (K/D, kills, win rate, dégâts, temps de survie, top-1, etc.) avec filtres et visualisations interactives

---

## Phase 1 : Configuration du Projet (Semaine 1)

### 1.1 Initialisation du projet Backend (Rust)
- [ ] Initialiser un projet Rust avec Cargo : `cargo new pubg-tracker-api`
- [ ] Configurer la structure de dossiers :
  ```
  backend/
    src/
      ├── main.rs              # Point d'entrée
      ├── config/              # Configuration (env, constantes)
      ├── models/              # Structures de données
      ├── services/            # Logique métier
      ├── handlers/            # Handlers HTTP
      ├── routes/              # Définition des routes
      ├── middleware/          # Middlewares (CORS, error handling)
      ├── db/                  # Repository pattern MongoDB
      └── utils/               # Utilitaires (retry logic, rate limiter)
    tests/
      ├── unit/                # Tests unitaires
      └── integration/         # Tests d'intégration
    Cargo.toml                 # Dépendances
  ```
- [ ] Ajouter les dépendances dans `Cargo.toml` :
  - `axum` ou `actix-web` pour le framework web
  - `tokio` pour async runtime
  - `mongodb` pour MongoDB driver
  - `reqwest` pour les appels HTTP (PUBG API)
  - `serde` & `serde_json` pour sérialisation
  - `dotenv` pour variables d'environnement
  - `tracing` & `tracing-subscriber` pour logs structurés
  - `tower` & `tower-http` pour middlewares (CORS, compression)
  - `validator` pour validation des données

### 1.2 Initialisation du projet Frontend (Next.js)
- [ ] Initialiser Next.js : `npx create-next-app@latest frontend --typescript --tailwind --app`
- [ ] Configurer la structure de dossiers :
  ```
  frontend/
    app/
      ├── page.tsx             # Page d'accueil
      ├── players/             # Pages joueurs
      │   ├── page.tsx         # Liste des joueurs
      │   └── [id]/            # Détails d'un joueur
      ├── layout.tsx           # Layout principal
      └── api/                 # API routes (optionnel)
    components/
      ├── PlayerCard.tsx
      ├── PlayerList.tsx
      └── MatchList.tsx
    lib/
      ├── api.ts               # Client API backend
      └── types.ts             # Types TypeScript
    public/                    # Assets statiques
  ```
- [ ] Installer les dépendances :
  - `axios` ou `fetch` pour appels API
  - `swr` ou `react-query` pour cache et state management
  - `zod` pour validation côté client
  - Tailwind CSS (déjà inclus)

### 1.3 Configuration de l'environnement
- [ ] Backend `.env.example` :
  ```
  RUST_ENV=development
  HOST=0.0.0.0
  PORT=8080
  MONGODB_URI=mongodb://localhost:27017/pubg-tracker
  PUBG_API_KEY=your_key_here
  PUBG_API_BASE_URL=https://api.pubg.com
  CORS_ORIGIN=http://localhost:3000
  RUST_LOG=info
  ```
- [ ] Frontend `.env.local.example` :
  ```
  NEXT_PUBLIC_API_URL=http://localhost:8080/api
  ```
- [ ] Configurer `.gitignore` (target/, node_modules/, .env, .next/)
- [ ] Configurer `rustfmt.toml` et `clippy` pour Rust
- [ ] Configurer ESLint et Prettier pour Next.js

---

## Phase 2 : Backend Rust - Modèles et Base de Données (Semaine 1-2)

### 2.1 Modèle de données MongoDB
- [ ] Créer la structure `Player` avec Serde :
  ```rust
  #[derive(Debug, Serialize, Deserialize, Clone)]
  pub struct Player {
      #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
      pub id: Option<ObjectId>,
      pub account_id: String,
      pub name: String,
      pub shard: String,
      pub last_matches: Vec<String>,
      pub last_refreshed_at: Option<DateTime>,
      pub created_at: DateTime,
      pub summary: Option<serde_json::Value>, // Données agrégées optionnelles
  }
  ```
- [ ] Créer la structure `PlayerStats` pour le cache des statistiques :
  ```rust
  #[derive(Debug, Serialize, Deserialize, Clone)]
  pub struct PlayerStats {
      #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
      pub id: Option<ObjectId>,
      pub player_id: String,
      pub period: String, // "last7d", "last30d", "last90d"
      pub mode: String,   // "solo", "duo", "squad"
      pub shard: String,
      pub kills: f64,
      pub deaths: f64,
      pub kd_ratio: f64,
      pub win_rate: f64,
      pub damage_dealt: f64,
      pub survival_time: f64,
      pub top1_count: u32,
      pub matches_played: u32,
      pub computed_at: DateTime,
      pub expires_at: DateTime, // TTL index
  }
  ```
- [ ] Créer les DTOs (Data Transfer Objects) pour les requêtes/réponses
- [ ] Implémenter la validation avec `validator` crate
- [ ] Configurer la connexion MongoDB avec pool de connexions
- [ ] Créer les index MongoDB :
  - `account_id` unique sur `players`
  - `{ player_id: 1, period: 1, mode: 1, shard: 1 }` sur `player_stats`
  - TTL index sur `expires_at` (collection `player_stats`)

### 2.2 Repository Pattern
- [ ] Créer `PlayerRepository` struct avec implémentation async :
  ```rust
  pub struct PlayerRepository {
      collection: Collection<Player>,
  }
  ```
- [ ] Implémenter les méthodes async :
  - `create(&self, player: Player) -> Result<Player>`
  - `find_by_id(&self, id: &str) -> Result<Option<Player>>`
  - `find_by_account_id(&self, account_id: &str) -> Result<Option<Player>>`
  - `find_all(&self, page: u64, limit: u64) -> Result<Vec<Player>>`
  - `update(&self, id: &str, player: Player) -> Result<Player>`
  - `delete(&self, id: &str) -> Result<()>`

---

## Phase 3 : Backend Rust - Services et Intégration API PUBG (Semaine 2-3)

### 3.1 Service PUBG API
- [ ] Créer `PubgApiService` struct avec client `reqwest` :
  ```rust
  pub struct PubgApiService {
      client: Client,
      api_key: String,
      base_url: String,
  }
  ```
- [ ] Implémenter les méthodes async :
  - `get_player_by_name(&self, name: &str, shard: &str) -> Result<String>` → accountId
  - `get_player_matches(&self, account_id: &str, shard: &str) -> Result<Vec<String>>` → match IDs
- [ ] Créer les structures de réponse PUBG avec Serde
- [ ] Gestion des erreurs avec custom `Error` enum (401, 403, 404, 429, 5xx)

### 3.2 Rate Limiting & Resilience
- [ ] Créer un module `retry` avec logique de backoff exponentiel
- [ ] Implémenter `RetryPolicy` :
  - Détecter status `429` et parser `X-RateLimit-Reset` header
  - Attendre jusqu'à reset time (avec `tokio::time::sleep`)
  - Backoff par défaut ≥ 6s si header absent
  - Maximum 3 tentatives
- [ ] Logger `X-RateLimit-Remaining` avec `tracing`
- [ ] Implémenter un cache avec `moka` ou `cached` (optionnel)

### 3.3 Service Métier Players
- [ ] Créer `PlayerService` struct :
  ```rust
  pub struct PlayerService {
      repository: PlayerRepository,
      pubg_api: PubgApiService,
      stats_service: StatsService,
  }
  ```
- [ ] Implémenter les méthodes async :
  - `add_player(&self, name: String, shard: String) -> Result<Player>`
  - `get_player_by_id(&self, id: &str) -> Result<Player>`
  - `list_players(&self, page: u64, limit: u64) -> Result<Vec<Player>>`
  - `refresh_player(&self, id: &str) -> Result<Vec<String>>`
  - `get_player_matches(&self, id: &str) -> Result<Vec<MatchInfo>>`

### 3.4 Service de Statistiques
- [ ] Créer `StatsService` struct :
  ```rust
  pub struct StatsService {
      stats_repository: StatsRepository,
      pubg_api: PubgApiService,
      cache: Arc<Mutex<LruCache<String, PlayerStats>>>, // Cache mémoire
  }
  ```
- [ ] Implémenter les méthodes de calcul des statistiques :
  - `get_player_stats(&self, player_id: &str, period: &str, mode: &str, shard: &str) -> Result<PlayerStats>`
  - `compute_stats_from_matches(&self, matches: Vec<Match>) -> Result<PlayerStats>`
  - `get_dashboard_stats(&self, player_ids: Vec<String>, period: &str, mode: &str, shard: &str) -> Result<Vec<PlayerStats>>`
  - `invalidate_cache(&self, player_id: &str)` - Appelé après refresh
- [ ] Implémenter la logique de cache :
  - Vérifier cache mémoire (TTL 60-300s)
  - Sinon vérifier `player_stats` collection en DB
  - Sinon calculer depuis les matches et mettre en cache
- [ ] Calculer les métriques :
  - K/D ratio : kills / deaths
  - Win rate : (wins / matches_played) × 100
  - Damage per match : total_damage / matches_played
  - Avg survival time : total_time / matches_played
  - Top-1 count : nombre de victoires

---

## Phase 4 : Backend Rust - API REST et Routes (Semaine 3)

### 4.1 Définition des routes (avec Axum ou Actix-web)
- [ ] Créer les handlers dans `src/handlers/player_handler.rs`
- [ ] `POST /api/players` - Ajouter un joueur
  - Body: `CreatePlayerRequest { name, shard }`
  - Validation avec `validator` crate
- [ ] `GET /api/players?page=1&limit=10` - Lister les joueurs
  - Query params avec Serde
- [ ] `GET /api/players/:id` - Détails d'un joueur
  - Path param extraction
- [ ] `POST /api/players/:id/refresh` - Rafraîchir les matches
  - Invalider le cache des stats après refresh
- [ ] `GET /api/players/:id/matches` - Lister les matches d'un joueur

- [ ] **Créer les handlers dashboard dans `src/handlers/dashboard_handler.rs`**
- [ ] `GET /api/dashboard?ids=id1,id2,...&period=last30d&mode=squad&shard=steam` - Dashboard comparatif
  - Query params : `ids` (liste), `period`, `mode`, `shard`
  - Retourne les statistiques agrégées pour comparer plusieurs joueurs
  - Exemple de réponse :
    ```json
    {
      "players": [
        {
          "player_id": "123",
          "name": "PlayerName",
          "stats": {
            "kills": 250,
            "kd_ratio": 2.5,
            "win_rate": 15.3,
            "damage_dealt": 85000,
            "survival_time": 12500,
            "top1_count": 8
          }
        }
      ],
      "period": "last30d",
      "mode": "squad"
    }
    ```
- [ ] `GET /api/players/:id/stats?period=last7d&mode=solo&shard=steam` - Statistiques détaillées d'un joueur
  - Retourne les stats pour graphiques et tooltips

### 4.2 Handlers
- [ ] Implémenter les handlers async :
  ```rust
  pub async fn create_player(
      State(service): State<Arc<PlayerService>>,
      Json(payload): Json<CreatePlayerRequest>,
  ) -> Result<Json<Player>, AppError>
  ```
- [ ] Créer les structures de requête/réponse avec validation
- [ ] Gestion des codes HTTP avec custom `AppError` enum (200, 201, 400, 404, 409, 429, 502)

### 4.3 Middlewares
- [ ] Middleware CORS avec `tower-http` :
  ```rust
  CorsLayer::new()
      .allow_origin(origin.parse::<HeaderValue>()?)
      .allow_methods([Method::GET, Method::POST])
  ```
- [ ] Middleware de logging avec `tracing` et `tower-http`
- [ ] Middleware d'erreurs global (conversion vers JSON)
- [ ] Middleware de rate limiting avec `tower::limit` (optionnel)
- [ ] Headers de sécurité (custom middleware)

---

## Phase 5 : Backend Rust - Observabilité et Logging ✅ TERMINÉ (Semaine 3-4)

### 5.1 Logs structurés avec Tracing ✅
- [x] Configurer `tracing` et `tracing-subscriber` :
  ```rust
  tracing_subscriber::fmt()
      .with_target(false)
      .with_level(true)
      .with_env_filter(log_level)
      .json()
      .init();
  ```
- [x] Utiliser les macros `info!`, `warn!`, `error!` avec spans
- [x] Créer des spans pour tracer les requêtes (#[tracing::instrument])
- [x] Logger les métriques :
  - Latence des endpoints (middleware logging)
  - Durée de traitement (duration_ms)
  - Status codes (middleware error)
  - Opérations services (debug logs)

### 5.2 Middlewares ✅
- [x] Middleware HTTP de traçage (logging.rs)
  - Logs de toutes les requêtes (method, uri, status, duration_ms)
- [x] Middleware de gestion des erreurs (error.rs)
  - Log automatique des 5xx (ERROR) et 4xx (WARN)
- [x] Middleware CORS configurable (cors.rs)
  - Mode développement (CORS_ORIGIN="*")
  - Mode production (origin restreint)

### 5.3 Instrumentation des Services ✅
- [x] `PubgApiService` :
  - #[tracing::instrument] sur get_player_by_name
  - Logs des requêtes API, retry, rate limits
- [x] `StatsService` :
  - #[tracing::instrument] sur get_or_compute_stats
  - Logs des cache hits/misses (mémoire et DB)
- [x] `PlayerService` :
  - #[tracing::instrument] sur add_player, refresh_player, delete_player
  - Logs des opérations CRUD

### 5.4 Configuration ✅
- [x] Variables d'environnement pour logging :
  - RUST_LOG=debug (développement)
  - RUST_LOG=info (production)
  - CORS_ORIGIN configurable

**📄 Documentation** : Voir [phase5_observability.md](phase5_observability.md) pour les détails complets

---

## Phase 6 : Tests Backend Rust ✅ TERMINÉ (Semaine 4)

### 6.1 Tests unitaires ✅
- [x] Configurer les tests avec `#[cfg(test)]` et `#[tokio::test]`
- [x] Tests sur `PubgApiService` avec mocking (mockito)
  - 5 tests : success, not_found, rate_limit, server_error_retry, max_retries_exceeded
  - ✅ Tous les tests passent (100% réussite)
- [x] Tests sur `StatsService` (3 tests préparés)
  - cache_operations, compute_stats_calculations, stats_ttl_expiration
- [x] Tests sur `PlayerService` (préparés)
- [x] Infrastructure complète avec lib.rs et common/mod.rs
- [x] Utilitaires de tests (mock_pubg_player_response, cleanup_test_data)

### 6.2 Tests d'intégration ✅
- [x] Configuration testcontainers pour MongoDB
- [x] 13 tests end-to-end préparés :
  - Health endpoint
  - Player CRUD (create, read, update, delete)
  - Dashboard stats avec filtres
  - Gestion des erreurs (404, 400, 409)
- [x] Tests des middlewares (CORS, error handling, logging)

### 6.3 CI/CD - Tests ✅
- [x] GitHub Actions workflow (`.github/workflows/backend-ci.yml`)
  - Job Test : fmt check, clippy, build, test
  - Job Coverage : cargo-tarpaulin + Codecov
  - MongoDB service container
  - Cache cargo (registry, index, build)
- [x] Déclencheurs : push/PR sur main/develop (paths: backend/**)

**📄 Documentation** : Voir [phase6_tests.md](phase6_tests.md) pour les détails complets

**Résultats** :
```
✅ PubgApiService : 5/5 tests passed in 9.07s
✅ Infrastructure CI/CD configurée
✅ Coverage tool intégré (cargo-tarpaulin)
```

---
    run: cargo tarpaulin --out Xml
  ```
- [ ] Ajouter `cargo clippy` pour linting
- [ ] Ajouter `cargo fmt --check` pour formatting

---

## Phase 7 : Frontend Next.js - Développement (Semaine 4-5)

### 7.1 Pages et Composants

#### Page d'accueil - Dashboard comparatif (`app/page.tsx`)
- [ ] **Fonctionnalité principale** : comparer 2+ joueurs avec métriques visuelles
- [ ] Sélection multi-joueurs (dropdown ou recherche autocomplete)
- [ ] **Filtres interactifs** :
  - Période : 7/30/90 jours (boutons radio ou tabs)
  - Shard : steam/xbox/psn (dropdown)
  - Mode de jeu : solo/duo/squad (tabs)
- [ ] **Visualisations** (au moins 2 types) :
  - Graphique en barres (Chart.js/Recharts) : comparaison des kills, dégâts, K/D
  - Radar chart : vue multi-métriques (kills, K/D, win rate, survie)
  - Leaderboard : tableau trié par métrique sélectionnée
- [ ] **Métriques affichées** (minimum 5) :
  - K/D ratio
  - Total kills
  - Win rate (%)
  - Dégâts moyens
  - Temps de survie moyen
  - Top-1 count
- [ ] Micro-interactions fluides (< 120ms pour changement de filtre)
- [ ] Formulaire d'ajout rapide de joueur en haut de page

#### Page liste des joueurs (`app/players/page.tsx`)
- [ ] Liste paginée avec `PlayerCard` composants
- [ ] Recherche/filtrage par nom, shard
- [ ] Tri par last_refreshed_at, created_at
- [ ] Action rapide : "Ajouter au dashboard"
- [ ] Statistiques en un coup d'œil par joueur

#### Page détails joueur (`app/players/[id]/page.tsx`)
- [ ] Informations du joueur
- [ ] Bouton refresh avec indicateur de progression
- [ ] Graphiques d'évolution des stats (timeline)
- [ ] Liste des matches récents avec détails
- [ ] Mini-dashboard personnel
- [ ] Lien vers télémétrie des matches

#### Composants de visualisation
- [ ] `ComparisonBarChart` : graphique en barres comparatif
- [ ] `RadarChart` : vue radar multi-métriques
- [ ] `StatsLeaderboard` : tableau de classement interactif
- [ ] `MetricCard` : carte de métrique avec icône et couleur
- [ ] `PlayerSelector` : sélecteur multi-joueurs avec recherche
- [ ] `FilterPanel` : panneau de filtres (période, mode, shard)

#### Composants réutilisables
- [ ] `PlayerCard` : affichage compact d'un joueur avec stats
- [ ] `PlayerList` : liste avec pagination
- [ ] `MatchList` : liste des matches avec liens
- [ ] `LoadingSpinner`, `ErrorAlert`, `EmptyState`
- [ ] `StatsBadge` : badge pour afficher une métrique

### 7.2 Client API et State Management
- [ ] Créer `lib/api.ts` avec fonctions async :
  ```typescript
  // Gestion des joueurs
  export async function addPlayer(name: string, shard: string): Promise<Player>
  export async function getPlayers(page: number, limit: number): Promise<Player[]>
  export async function getPlayer(id: string): Promise<Player>
  export async function refreshPlayer(id: string): Promise<string[]>
  export async function getPlayerMatches(id: string): Promise<Match[]>
  
  // Dashboard et statistiques
  export async function getDashboardStats(
    playerIds: string[],
    period: string,
    mode: string,
    shard: string
  ): Promise<DashboardData>
  
  export async function getPlayerStats(
    playerId: string,
    period: string,
    mode: string,
    shard: string
  ): Promise<PlayerStats>
  ```
- [ ] Créer `lib/types.ts` avec interfaces TypeScript :
  ```typescript
  export interface Player {
    id: string
    account_id: string
    name: string
    shard: string
    last_matches: string[]
    last_refreshed_at?: Date
    created_at: Date
  }
  
  export interface PlayerStats {
    player_id: string
    kills: number
    deaths: number
    kd_ratio: number
    win_rate: number
    damage_dealt: number
    survival_time: number
    top1_count: number
    matches_played: number
  }
  
  export interface DashboardData {
    players: Array<{
      player_id: string
      name: string
      stats: PlayerStats
    }>
    period: string
    mode: string
  }
  ```
- [ ] Intégrer SWR ou React Query pour :
  - Cache automatique avec revalidation
  - Stale-while-revalidate pour UX fluide
  - Loading states
  - Error handling
  - Optimistic updates
  - Polling pour refresh automatique (optionnel)
- [ ] Créer des hooks personnalisés :
  - `useDashboard(playerIds, filters)` - stats comparées
  - `usePlayerStats(playerId, filters)` - stats d'un joueur
  - `usePlayers()` - liste des joueurs
  - `usePlayer(id)` - détails d'un joueur

### 7.3 UI/UX et Visualisations
- [ ] Installer les dépendances de visualisation :
  - `recharts` ou `chart.js` + `react-chartjs-2` pour les graphiques
  - `lucide-react` pour les icônes
  - `clsx` et `tailwind-merge` pour la gestion des classes CSS
  - `framer-motion` pour animations (optionnel)
- [ ] Design responsive avec Tailwind CSS :
  - Mobile-first (≥ 320px)
  - Breakpoints : sm (640px), md (768px), lg (1024px), xl (1280px)
  - Grilles adaptatives pour les cartes de métriques
- [ ] Thème "ludique" mais professionnel :
  - Palette de couleurs contrastante (accessibilité AA)
  - Dégradés pour les cartes de métriques
  - Animations subtiles (transitions, hover effects)
  - Micro-interactions pour les filtres (< 120ms)
- [ ] Gestion des états de chargement :
  - Skeletons pour les graphiques
  - Spinners pour les actions
  - Progressive loading pour les grandes listes
  - Shimmer effects
- [ ] Gestion des erreurs :
  - Toasts pour notifications (sonner ou react-hot-toast)
  - Messages d'erreur contextuels
  - Retry automatique avec backoff
  - États vides avec illustrations
- [ ] Accessibilité (WCAG AA) :
  - Navigation clavier complète
  - ARIA labels sur les graphiques et composants interactifs
  - Focus visibles avec ring
  - Contrastes AA minimum (4.5:1 texte, 3:1 UI)
  - Screen reader friendly

### 7.4 Tests Frontend
- [ ] Tests unitaires avec Jest + React Testing Library :
  - Composants de visualisation
  - Hooks personnalisés
  - Fonctions utilitaires
  - Tests de snapshot pour les composants UI
- [ ] Tests d'intégration :
  - Flux complet d'ajout de joueur
  - Dashboard avec changement de filtres
  - Gestion des états de chargement et d'erreur
- [ ] Tests E2E avec Playwright (optionnel) :
  - Parcours utilisateur complet
  - Ajouter joueur → voir dashboard → comparer → rafraîchir
  - Tests cross-browser (Chrome, Firefox, Safari)

---

## Phase 8 : Documentation API (Semaine 5)

### 8.1 Documentation Backend
- [ ] Créer un fichier OpenAPI (Swagger) 3.0 ou utiliser `utoipa` crate :
  ```rust
  #[utoipa::path(
      post,
      path = "/api/players",
      request_body = CreatePlayerRequest,
      responses(
          (status = 200, description = "Player created", body = Player),
          (status = 404, description = "Player not found")
      )
  )]
  ```
- [ ] Générer Swagger UI automatiquement avec `utoipa-swagger-ui`
- [ ] Documenter tous les endpoints avec :
  - Schémas de requête/réponse
  - Codes d'erreur
  - Exemples

### 8.2 Documentation Frontend
- [ ] Documenter les composants avec JSDoc
- [ ] Créer un Storybook (optionnel) pour visualiser les composants

### 8.3 README
- [ ] Backend README.md :
  - Architecture
  - Installation (Rust, MongoDB)
  - Configuration (.env)
  - Lancement local : `cargo run`
  - Tests : `cargo test`
  - Exemples d'utilisation de l'API
- [ ] Frontend README.md :
  - Installation : `npm install`
  - Configuration (.env.local)
  - Lancement : `npm run dev`
  - Build : `npm run build`
- [ ] README principal avec aperçu du monorepo

---

## Phase 9 : Conteneurisation (Semaine 5-6)

### 9.1 Dockerfile Backend (Rust)
- [ ] Créer `backend/Dockerfile` multi-stage :
  ```dockerfile
  # Stage 1: Build
  FROM rust:1.75-alpine AS builder
  RUN apk add --no-cache musl-dev openssl-dev
  WORKDIR /app
  COPY Cargo.toml Cargo.lock ./
  COPY src ./src
  RUN cargo build --release
  
  # Stage 2: Production
  FROM alpine:latest
  RUN apk add --no-cache libgcc openssl
  WORKDIR /app
  COPY --from=builder /app/target/release/pubg-tracker-api .
  EXPOSE 8080
  CMD ["./pubg-tracker-api"]
  ```
- [ ] Créer `.dockerignore` pour backend

### 9.2 Dockerfile Frontend (Next.js)
- [ ] Créer `frontend/Dockerfile` multi-stage :
  ```dockerfile
  # Stage 1: Dependencies
  FROM node:20-alpine AS deps
  WORKDIR /app
  COPY package*.json ./
  RUN npm ci
  
  # Stage 2: Build
  FROM node:20-alpine AS builder
  WORKDIR /app
  COPY --from=deps /app/node_modules ./node_modules
  COPY . .
  RUN npm run build
  
  # Stage 3: Production
  FROM node:20-alpine AS runner
  WORKDIR /app
  ENV NODE_ENV production
  COPY --from=builder /app/public ./public
  COPY --from=builder /app/.next/standalone ./
  COPY --from=builder /app/.next/static ./.next/static
  EXPOSE 3000
  CMD ["node", "server.js"]
  ```
- [ ] Configurer `next.config.js` avec `output: 'standalone'`
- [ ] Créer `.dockerignore` pour frontend

### 9.3 Docker Compose (développement local)
- [ ] Créer `docker-compose.yml` à la racine :
  ```yaml
  services:
    backend:
      build: ./backend
      ports:
        - "8080:8080"
      environment:
        - MONGODB_URI=mongodb://mongo:27017/pubg-tracker
        - PUBG_API_KEY=${PUBG_API_KEY}
      depends_on:
        - mongo
    
    frontend:
      build: ./frontend
      ports:
        - "3000:3000"
      environment:
        - NEXT_PUBLIC_API_URL=http://localhost:8080/api
      depends_on:
        - backend
    
    mongo:
      image: mongo:7
      ports:
        - "27017:27017"
      volumes:
        - mongo-data:/data/db
  
  volumes:
    mongo-data:
  ```
- [ ] Tester la construction et le lancement : `docker-compose up`

---

## Phase 10 : Déploiement Azure (Semaine 6-7)

### 10.1 Préparation des ressources Azure
- [ ] **Azure Container Registry (ACR)** :
  - Créer un registre pour stocker les images Docker (backend + frontend)
  - SKU : Standard ou Premium
- [ ] **Azure Cosmos DB for MongoDB** :
  - Créer une instance Cosmos DB avec API MongoDB
  - Configurer les performances (RU/s : début avec 400-1000)
  - Activer l'auto-scaling si nécessaire
  - Récupérer la chaîne de connexion
- [ ] **Azure Container Apps** pour le backend Rust :
  - Environnement managé, scaling automatique
  - Support natif des conteneurs
- [ ] **Azure Static Web Apps** ou **Azure Container Apps** pour le frontend Next.js :
  - **Option A** : Static Web Apps (si SSG uniquement)
  - **Option B** : Container Apps (si SSR/ISR nécessaire) — RECOMMANDÉ

### 10.2 Infrastructure as Code (Bicep)
- [ ] Créer `infra/main.bicep` avec :
  - Resource Group
  - Azure Container Registry
  - Azure Cosmos DB (MongoDB API)
  - Azure Container Apps Environment (partagé)
  - Azure Container App pour backend (pubg-api)
  - Azure Container App pour frontend (pubg-web)
  - Azure Key Vault (pour les secrets)
  - Azure Application Insights (monitoring)
  - Azure Log Analytics Workspace
- [ ] Créer `infra/parameters.json` pour les valeurs configurables
- [ ] Exemple de structure Bicep pour Container App :
  ```bicep
  resource backendApp 'Microsoft.App/containerApps@2023-05-01' = {
    name: 'pubg-tracker-api'
    location: location
    properties: {
      managedEnvironmentId: environment.id
      configuration: {
        ingress: {
          external: true
          targetPort: 8080
        }
        secrets: [
          {
            name: 'pubg-api-key'
            keyVaultUrl: keyVault.properties.vaultUri
          }
        ]
      }
      template: {
        containers: [
          {
            name: 'api'
            image: '${acr.properties.loginServer}/pubg-api:latest'
            resources: {
              cpu: '0.5'
              memory: '1Gi'
            }
            env: [
              { name: 'MONGODB_URI', secretRef: 'mongodb-uri' }
              { name: 'PUBG_API_KEY', secretRef: 'pubg-api-key' }
            ]
          }
        ]
        scale: {
          minReplicas: 1
          maxReplicas: 5
        }
      }
    }
  }
  ```

### 10.3 Configuration des secrets
- [ ] Créer un Azure Key Vault
- [ ] Stocker les secrets :
  - `PUBG-API-KEY`
  - `MONGODB-URI` (Cosmos DB connection string)
- [ ] Configurer l'accès via Managed Identity :
  - Activer System-Assigned Identity sur les Container Apps
  - Donner les permissions `Key Vault Secrets User` aux identities

### 10.4 CI/CD Pipeline (GitHub Actions)
- [ ] Créer `.github/workflows/deploy-backend.yml` :
  ```yaml
  name: Deploy Backend to Azure
  
  on:
    push:
      branches: [main]
      paths:
        - 'backend/**'
  
  jobs:
    build-and-deploy:
      runs-on: ubuntu-latest
      steps:
        - uses: actions/checkout@v4
        
        - name: Log in to Azure
          uses: azure/login@v1
          with:
            creds: ${{ secrets.AZURE_CREDENTIALS }}
        
        - name: Log in to ACR
          run: az acr login --name ${{ secrets.ACR_NAME }}
        
        - name: Build and push Docker image
          working-directory: ./backend
          run: |
            docker build -t ${{ secrets.ACR_NAME }}.azurecr.io/pubg-api:${{ github.sha }} .
            docker push ${{ secrets.ACR_NAME }}.azurecr.io/pubg-api:${{ github.sha }}
            docker tag ${{ secrets.ACR_NAME }}.azurecr.io/pubg-api:${{ github.sha }} ${{ secrets.ACR_NAME }}.azurecr.io/pubg-api:latest
            docker push ${{ secrets.ACR_NAME }}.azurecr.io/pubg-api:latest
        
        - name: Deploy to Azure Container Apps
          run: |
            az containerapp update \
              --name pubg-tracker-api \
              --resource-group pubg-tracker-rg \
              --image ${{ secrets.ACR_NAME }}.azurecr.io/pubg-api:${{ github.sha }}
  ```
- [ ] Créer `.github/workflows/deploy-frontend.yml` (similaire pour frontend)
- [ ] Configurer les secrets GitHub :
  - `AZURE_CREDENTIALS` (service principal JSON)
  - `ACR_NAME` (nom du registre)

### 10.5 Configuration Backend Container App
- [ ] Variables d'environnement :
  - `RUST_ENV=production`
  - `HOST=0.0.0.0`
  - `PORT=8080`
  - `MONGODB_URI` (depuis Key Vault)
  - `PUBG_API_KEY` (depuis Key Vault)
  - `CORS_ORIGIN` (URL du frontend)
  - `RUST_LOG=info`
- [ ] Health probes :
  - Liveness: `/health`
  - Readiness: `/ready`
  - Initial delay: 10s, period: 30s
- [ ] Scaling :
  - Min replicas: 1
  - Max replicas: 5
  - Rules: HTTP requests (ex: 100 concurrent requests par replica)

### 10.6 Configuration Frontend Container App
- [ ] Variables d'environnement :
  - `NODE_ENV=production`
  - `NEXT_PUBLIC_API_URL` (URL du backend Container App)
- [ ] Health probes :
  - Liveness: `/` (page d'accueil)
  - Initial delay: 5s
- [ ] Scaling :
  - Min replicas: 1
  - Max replicas: 3

### 10.7 Networking et Sécurité
- [ ] Configurer le domaine personnalisé (optionnel) :
  - Backend API : `api.pubg-tracker.com`
  - Frontend : `pubg-tracker.com`
- [ ] Activer HTTPS (automatique avec Container Apps)
- [ ] Configurer CORS en production :
  - Backend autorise uniquement l'URL du frontend
- [ ] Configurer les règles de pare-feu Cosmos DB :
  - Autoriser Azure services
  - Restreindre l'accès par IP si possible

---

## Phase 11 : Monitoring et Observabilité Azure (Semaine 7-8)

### 11.1 Application Insights
- [ ] Configurer Application Insights pour le backend Rust :
  - Utiliser `azure-monitor` ou instrumentation custom via HTTP
  - Ou envoyer les logs structurés vers Log Analytics directement
- [ ] Configurer Application Insights pour Next.js :
  - Installer `@microsoft/applicationinsights-web`
  - Tracker les pages vues, erreurs client, performances
- [ ] Tracker les métriques personnalisées :
  - Latence des endpoints backend
  - Taux de succès/échec des appels PUBG API
  - X-RateLimit-Remaining
  - Nombre de retry 429
  - Temps de réponse frontend

### 11.2 Azure Monitor
- [ ] Configurer des alertes :
  - Taux d'erreur backend > 5%
  - Latence p95 > 1s
  - Rate limit PUBG proche de 0 (X-RateLimit-Remaining < 10)
  - Erreurs Cosmos DB
  - Réplicas Container Apps = max (saturation)
  - Erreurs JavaScript côté client > seuil
- [ ] Créer un dashboard Azure avec :
  - Requêtes par seconde (backend + frontend)
  - Latence moyenne/p95
  - Taux d'erreur
  - Santé des services (PUBG API, Cosmos DB)
  - Utilisation RU/s Cosmos DB
  - Nombre de réplicas actifs

### 11.3 Logs
- [ ] Configurer la collecte des logs dans Log Analytics Workspace
- [ ] Backend Rust : logs JSON structurés avec `tracing`
- [ ] Frontend Next.js : logs server-side + erreurs client
- [ ] Créer des requêtes KQL pour :
  - Erreurs par endpoint
  - Traces de requêtes avec correlation ID
  - Métriques de performance
  - Analyse des erreurs PUBG API

---

## Phase 12 : Optimisations et Améliorations (Semaine 7-8)

### 12.1 Performance Backend
- [ ] Implémenter un cache Redis (Azure Cache for Redis) :
  - Cache des résultats PUBG API (5-10 min TTL)
  - **Cache des statistiques calculées (60-300s TTL)** - PRIORITAIRE pour dashboard
  - Réduire les appels API et calculs coûteux
  - Structure de clé : `stats:{player_id}:{period}:{mode}:{shard}`
- [ ] Optimiser les requêtes MongoDB :
  - Index composites pour les requêtes fréquentes
  - Projection pour limiter les données retournées
  - Agrégations optimisées (pipeline stages efficaces)
- [ ] Implémenter le cache en mémoire avec `moka` :
  - LRU cache pour les stats les plus demandées
  - Fallback vers Redis si miss
  - TTL configurable par type de données
- [ ] Optimiser les calculs de statistiques :
  - Calcul incrémental plutôt que recalcul complet
  - Parallélisation avec `tokio::spawn` pour plusieurs joueurs
  - Batch processing pour le dashboard multi-joueurs

### 12.2 Performance Frontend
- [ ] Optimiser les rendus Next.js :
  - Utiliser React.memo pour les composants de graphiques
  - Virtualisation pour les longues listes (react-virtual)
  - Code splitting avec dynamic imports
  - Image optimization avec next/image
- [ ] Optimiser les appels API :
  - Debouncing pour les recherches
  - Batching des requêtes dashboard
  - Prefetching avec SWR
  - Cache stale-while-revalidate

### 12.3 Scalabilité
- [ ] Tester le scaling automatique avec charge :
  - Tests avec Azure Load Testing
  - Mesurer le temps de scale-up/down
  - Vérifier les seuils de scaling
- [ ] Optimiser les RU/s Cosmos DB selon l'usage réel :
  - Analyser les métriques de consommation
  - Ajuster le provisioning
  - Activer l'auto-scaling si nécessaire
- [ ] Implémenter pagination efficace :
  - Cursor-based pagination si volume important
  - Pagination côté serveur pour dashboard
  - Lazy loading des graphiques

### 12.4 Sécurité
- [ ] Audit de sécurité :
  - Vérifier qu'aucun secret n'est exposé (PUBG_API_KEY notamment)
  - Tester les headers de sécurité (helmet equivalent en Rust)
  - Valider CORS en production (uniquement domaine frontend)
  - Scanner les vulnérabilités avec `cargo audit`
- [ ] Implémenter rate limiting applicatif :
  - Limiter les requêtes par IP avec `tower::limit`
  - Protection contre les abus du endpoint dashboard
  - Quotas par utilisateur si authentification future
- [ ] Sécuriser les endpoints sensibles :
  - Authentification pour refresh (optionnel pour MVP)
  - Validation stricte des paramètres
  - Protection CSRF si formulaires

---

## Phase 13 : Documentation et Formation (Semaine 8)

### 13.1 Documentation technique
- [ ] README.md complet :
  - **Architecture** (diagramme backend + frontend + Azure)
  - **Dashboard comparatif** : fonctionnalités, métriques, filtres
  - Guide de développement local
  - Guide de déploiement Azure
  - Troubleshooting (rate limit PUBG, cache, erreurs courantes)
- [ ] Documentation d'architecture :
  - Diagrammes (architecture Azure, flux de données, calcul des stats)
  - Décisions techniques (ADRs) :
    - Choix de Rust pour performance
    - Stratégie de cache à plusieurs niveaux
    - Gestion du rate limit PUBG
    - Cosmos DB vs MongoDB Atlas
- [ ] Guide d'opérations :
  - **Monitoring des statistiques** (cache hit rate, temps de calcul)
  - Gestion des alertes
  - Procédures d'incident
  - Rotation des secrets (Key Vault)
  - Scaling manuel vs automatique
- [ ] Documentation API :
  - **Endpoints dashboard** détaillés
  - Exemples de requêtes/réponses
  - Guide d'intégration

### 13.2 Documentation utilisateur
- [ ] Guide utilisateur dashboard :
  - Comment comparer des joueurs
  - Interprétation des métriques (K/D, win rate, etc.)
  - Utilisation des filtres
  - Lecture des graphiques
- [ ] FAQ :
  - Pourquoi mes stats ne se mettent pas à jour ?
  - Comment ajouter un joueur ?
  - Que signifie "rate limit" ?

### 13.3 Passation
- [ ] Session de démonstration :
  - **Démo du dashboard comparatif**
  - Flux complet utilisateur
  - Administration Azure
- [ ] Documentation des accès et credentials
- [ ] Guide de maintenance :
  - Mise à jour des dépendances
  - Déploiement de nouvelles versions
  - Gestion du cache

---

## Phase 14 : Post-Déploiement (Semaine 8-9)

### 14.1 Tests de production
- [ ] Tests de bout en bout en production :
  - Ajouter un joueur réel
  - Tester le dashboard avec plusieurs joueurs
  - Vérifier tous les filtres (période, mode, shard)
  - Valider les visualisations (graphiques, leaderboard)
  - Tester le refresh avec gestion du rate limit
- [ ] Tests de charge (Azure Load Testing) :
  - Simuler 50-100 utilisateurs simultanés sur dashboard
  - Mesurer les temps de réponse (p50, p95, p99)
  - Vérifier le comportement du cache
  - Tester le scaling automatique
- [ ] Validation des alertes :
  - Déclencher intentionnellement des erreurs
  - Vérifier la réception des alertes
  - Tester les runbooks de réponse

### 14.2 Optimisation continue
- [ ] Analyser les métriques de production :
  - **Cache hit rate** (objectif > 80% pour les stats)
  - Temps de calcul des statistiques
  - Latence du dashboard (objectif < 500ms p95)
  - Consommation RU/s Cosmos DB
  - Nombre d'appels PUBG API (respecter 10 req/min)
- [ ] Ajuster le scaling si nécessaire :
  - Réviser les seuils de scaling
  - Optimiser min/max replicas
  - Ajuster les ressources (CPU/RAM)
- [ ] Optimiser les coûts Azure :
  - Analyser les ressources sous-utilisées
  - Ajuster les tiers de services
  - Activer auto-scaling Cosmos DB si pertinent
  - Optimiser le cache Redis (tier approprié)

### 14.3 Feedback et améliorations
- [ ] Collecter le feedback utilisateur :
  - Facilité d'utilisation du dashboard
  - Pertinence des métriques affichées
  - Performance ressentie
  - Fonctionnalités manquantes
- [ ] Planifier les améliorations futures :
  - Nouvelles visualisations
  - Métriques additionnelles
  - Comparaison d'équipes (au-delà de joueurs individuels)
  - Historique d'évolution des stats
  - Notifications de changements significatifs

---

## Architecture Azure Recommandée

```
┌─────────────────────────────────────────────────────────────────────┐
│                            AZURE                                     │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │  Azure Container Apps Environment (Shared)                     │ │
│  │                                                                 │ │
│  │  ┌─────────────────────────┐  ┌──────────────────────────────┐ │ │
│  │  │  Backend (Rust API)     │  │  Frontend (Next.js)          │ │ │
│  │  │  - Axum/Actix-web       │  │  - React + TypeScript        │ │ │
│  │  │  - Auto-scaling (1-5)   │  │  - SSR/ISR                   │ │ │
│  │  │  - Port 8080            │  │  - Auto-scaling (1-3)        │ │ │
│  │  │  - Managed Identity     │  │  - Port 3000                 │ │ │
│  │  └────────┬────────────────┘  └──────────────┬───────────────┘ │ │
│  │           │ API Calls                         │                 │ │
│  │           └───────────────────────────────────┘                 │ │
│  └────────────┬────────────────────────────────────┬──────────────┘ │
│               │ Secrets                            │ Data           │
│               ▼                                    ▼                │
│  ┌─────────────────────┐              ┌────────────────────────┐   │
│  │  Azure Key Vault    │              │  Azure Cosmos DB       │   │
│  │  - PUBG_API_KEY     │              │  (MongoDB API)         │   │
│  │  - MONGODB_URI      │              │  - Players Collection  │   │
│  │  - Managed Identity │              │  - Auto-indexing       │   │
│  └─────────────────────┘              │  - 400-1000 RU/s       │   │
│                                        └────────────────────────┘   │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │  Azure Application Insights + Log Analytics                    │ │
│  │  - Backend logs (Rust tracing → JSON)                          │ │
│  │  - Frontend logs (Next.js SSR + Client)                        │ │
│  │  - Métriques : latency, errors, PUBG rate limit                │ │
│  │  - Alertes                                                      │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │  Azure Container Registry (ACR)                                │ │
│  │  - pubg-api:latest (Rust backend image)                        │ │
│  │  - pubg-web:latest (Next.js frontend image)                    │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                                                                      │
│  Optionnel:                                                         │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │  Azure Cache for Redis                                         │ │
│  │  - Cache PUBG API responses (5-10 min TTL)                     │ │
│  └────────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
                           ▲
                           │ HTTPS
                           │
                    ┌──────┴──────┐
                    │   Clients   │
                    │  (Browser)  │
                    └─────────────┘
```

---

## Estimation des coûts Azure (mensuel)

### Backend + Frontend
- **Azure Container Apps** (Backend Rust) : ~30-50€ (1-2 replicas, 0.5 CPU, 1 GB RAM)
- **Azure Container Apps** (Frontend Next.js) : ~25-40€ (1-2 replicas, 0.5 CPU, 1 GB RAM)
- **Azure Cosmos DB** (MongoDB API) : ~25-100€ (400-1000 RU/s selon usage)
- **Azure Container Registry** : ~5€ (Basic tier, < 10 GB)
- **Azure Key Vault** : ~1€ (secrets + transactions)
- **Application Insights + Log Analytics** : ~10-20€ (selon volume de logs et métriques)
- **Azure Cache for Redis** (optionnel) : ~15€ (Basic C0 tier)

**Total estimé** : ~100-230€/mois (selon usage et scaling)

💡 **Optimisations possibles** :
- Utiliser Azure Static Web Apps (gratuit jusqu'à 100 GB bandwidth) si SSG uniquement
- Commencer avec 400 RU/s Cosmos DB et activer auto-scaling
- Utiliser le tier gratuit d'Application Insights (5 GB/mois)

---

## Stack Technologique Finale

### Backend
- **Language** : Rust 1.75+
- **Framework** : Axum ou Actix-web
- **Async Runtime** : Tokio
- **Database Driver** : MongoDB official driver
- **HTTP Client** : Reqwest
- **Serialization** : Serde
- **Validation** : Validator
- **Logging** : Tracing + tracing-subscriber
- **Tests** : Cargo test + testcontainers

### Frontend
- **Framework** : Next.js 14+ (App Router)
- **Language** : TypeScript
- **UI Library** : React 18+
- **Styling** : Tailwind CSS
- **State Management** : SWR ou React Query
- **HTTP Client** : Fetch API ou Axios
- **Validation** : Zod
- **Tests** : Jest + React Testing Library + Playwright (E2E)

### Infrastructure Azure
- **Compute** : Azure Container Apps (backend + frontend)
- **Database** : Azure Cosmos DB for MongoDB
- **Registry** : Azure Container Registry
- **Secrets** : Azure Key Vault
- **Monitoring** : Azure Application Insights + Azure Monitor + Log Analytics
- **Cache (optionnel)** : Azure Cache for Redis
- **IaC** : Bicep (recommandé) ou Terraform

### DevOps
- **CI/CD** : GitHub Actions
- **Containerization** : Docker (multi-stage builds)
- **Version Control** : Git + GitHub
- **Linting** : Clippy (Rust) + ESLint (TypeScript)

---

## Checklist de Déploiement Final

### Backend
- [ ] Code backend Rust complet et testé (≥80% couverture)
- [ ] Tous les endpoints implémentés (players + **dashboard**)
- [ ] Service de statistiques fonctionnel avec cache
- [ ] Gestion du rate limit PUBG (10 req/min)
- [ ] Logs structurés avec tracing
- [ ] Tests unitaires et d'intégration passants

### Frontend
- [ ] **Dashboard comparatif fonctionnel** avec visualisations
- [ ] Au moins 2 types de graphiques (barres + radar/leaderboard)
- [ ] Filtres interactifs (période, mode, shard)
- [ ] 5+ métriques affichées (K/D, kills, win rate, dégâts, survie, top-1)
- [ ] Design responsive (mobile-first)
- [ ] Accessibilité AA (contrastes, navigation clavier)
- [ ] Tests unitaires et E2E

### Infrastructure Azure
- [ ] Image Docker backend construite et poussée vers ACR
- [ ] Image Docker frontend construite et poussée vers ACR
- [ ] Infrastructure Azure provisionnée (Bicep/Terraform)
- [ ] 2 Container Apps déployées (backend + frontend)
- [ ] Cosmos DB (MongoDB API) configuré avec index
- [ ] Secrets configurés dans Key Vault (PUBG_API_KEY, MONGODB_URI)
- [ ] Managed Identity activée et permissions configurées
- [ ] Variables d'environnement configurées dans Container Apps
- [ ] Health probes configurés et fonctionnels
- [ ] CORS configuré pour la production (frontend → backend)

### Monitoring et Observabilité
- [ ] Application Insights intégré (backend + frontend)
- [ ] Logs visibles dans Log Analytics
- [ ] Alertes configurées dans Azure Monitor :
  - Taux d'erreur > 5%
  - Latence p95 > 1s
  - Rate limit PUBG proche
  - Cache hit rate < 70%
- [ ] Dashboard Azure avec métriques clés
- [ ] Requêtes KQL créées pour diagnostics

### CI/CD
- [ ] Pipeline GitHub Actions backend testé et fonctionnel
- [ ] Pipeline GitHub Actions frontend testé et fonctionnel
- [ ] Déploiement automatique sur push main
- [ ] Tests exécutés automatiquement

### Documentation
- [ ] README principal avec overview complet
- [ ] Documentation du dashboard comparatif
- [ ] Guide de développement local
- [ ] Guide de déploiement Azure
- [ ] Documentation API (OpenAPI/Swagger)
- [ ] Documentation d'architecture avec diagrammes
- [ ] Guide d'opérations

### Sécurité
- [ ] Aucun secret en clair dans le code
- [ ] PUBG_API_KEY jamais exposée dans logs/frontend
- [ ] CORS restreint au domaine production
- [ ] Headers de sécurité configurés
- [ ] RBAC Azure configuré (moindre privilège)
- [ ] Scan de vulnérabilités passé (cargo audit)

### Performance
- [ ] Cache Redis configuré (optionnel mais recommandé)
- [ ] Collection stats_cache avec TTL
- [ ] Temps de réponse dashboard < 500ms p95
- [ ] Temps de calcul stats optimisé
- [ ] Tests de charge validés

### Tests de production
- [ ] Ajout de joueur testé
- [ ] Dashboard multi-joueurs testé
- [ ] Tous les filtres validés
- [ ] Refresh avec rate limit vérifié
- [ ] Visualisations correctes
- [ ] Tests cross-browser (Chrome, Firefox, Safari)
- [ ] Tests mobile (iOS, Android)

---

## Prochaines Étapes Recommandées

1. **Phase 1** - Initialiser les deux projets (backend Rust + frontend Next.js)
2. **Phase 2-4** - Développer le backend Rust en local avec MongoDB local
3. **Phase 7** - Développer le frontend Next.js en parallèle
4. **Phase 6** - Implémenter les tests (backend + frontend)
5. **Phase 9** - Conteneuriser les deux applications
6. **Phase 10** - Provisionner l'infrastructure Azure et déployer
7. **Phase 11** - Configurer monitoring et alertes
8. **Phase 12-14** - Optimiser selon les métriques de production

### Ordre de développement recommandé

**Semaine 1-2** : Backend Rust foundation
- Setup projet Rust avec structure
- Connexion MongoDB
- Modèles et repositories
- Service PUBG API avec retry logic

**Semaine 2-3** : Backend API complet
- Routes et handlers
- Middlewares (CORS, errors, logging)
- Tests unitaires et intégration
- Documentation OpenAPI

**Semaine 3-4** : Frontend Next.js
- Setup Next.js avec structure
- Pages (home, players list, player details)
- Composants réutilisables
- Client API avec SWR/React Query
- UI responsive avec Tailwind

**Semaine 5-6** : Docker + Azure
- Dockerfiles pour backend et frontend
- Docker Compose pour tests locaux
- Infrastructure Bicep
- Déploiement sur Azure
- CI/CD pipelines

**Semaine 7-8** : Monitoring et optimisation
- Application Insights intégration
- Alertes et dashboards
- Tests de charge
- Optimisations performance
- Documentation finale

---

## Commandes rapides pour démarrer

### Backend (Rust)
```bash
# Initialiser le projet
cargo new backend --name pubg-tracker-api
cd backend

# Ajouter les dépendances principales dans Cargo.toml
# Voir Phase 1.1 pour la liste complète

# Lancer en mode dev
cargo run

# Lancer les tests
cargo test

# Build optimisé
cargo build --release
```

### Frontend (Next.js)
```bash
# Initialiser le projet
npx create-next-app@latest frontend --typescript --tailwind --app

cd frontend

# Installer les dépendances
npm install axios swr

# Lancer en mode dev
npm run dev

# Build pour production
npm run build
npm start
```

### Docker (local)
```bash
# Build et lancer tout l'environnement
docker-compose up --build

# Backend accessible sur http://localhost:8080
# Frontend accessible sur http://localhost:3000
# MongoDB accessible sur localhost:27017
```

### Azure (déploiement)
```bash
# Se connecter à Azure
az login

# Déployer l'infrastructure
az deployment group create \
  --resource-group pubg-tracker-rg \
  --template-file infra/main.bicep \
  --parameters infra/parameters.json

# Push les images vers ACR
az acr login --name <acr-name>
docker push <acr-name>.azurecr.io/pubg-api:latest
docker push <acr-name>.azurecr.io/pubg-web:latest
```

---

## Contact et Support

- Documentation PUBG API : https://documentation.pubg.com/
- Documentation Azure Container Apps : https://learn.microsoft.com/azure/container-apps/
- Documentation Cosmos DB : https://learn.microsoft.com/azure/cosmos-db/

---

**Date de création** : 2026-02-09  
**Version** : 1.0  
**Statut** : Prêt pour implémentation
