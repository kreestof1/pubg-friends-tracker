# Phase 5 : Observabilité et Logging

## Objectifs de la Phase 5

- ✅ Logging structuré avec tracing
- ✅ Middleware de traçage HTTP
- ✅ Middleware de gestion des erreurs
- ✅ Configuration CORS avancée
- ✅ Instrumentation des services

## Architecture de l'Observabilité

### 1. Logging Structuré

Le logging utilise `tracing` avec le format JSON pour une intégration facile avec Azure Application Insights.

**Configuration dans main.rs :**
```rust
tracing_subscriber::fmt()
    .with_target(false)
    .with_level(true)
    .with_env_filter(log_level)
    .json()
    .init();
```

**Variables d'environnement :**
- `RUST_LOG=debug` : Logs détaillés (développement)
- `RUST_LOG=info` : Logs standards (production)
- `RUST_LOG=warn` : Warnings uniquement

### 2. Middleware HTTP

#### Middleware de Traçage (`logging.rs`)
Enregistre toutes les requêtes HTTP avec :
- Méthode HTTP (GET, POST, etc.)
- URI de la requête
- Status code de la réponse
- Durée de traitement en millisecondes

**Exemple de log :**
```json
{
  "timestamp": "2024-01-15T10:30:45Z",
  "level": "INFO",
  "message": "Request completed",
  "fields": {
    "method": "GET",
    "uri": "/api/players",
    "status": 200,
    "duration_ms": 45
  }
}
```

#### Middleware d'Erreur (`error.rs`)
Log automatique des erreurs selon le status code :
- **5xx** : Logged comme ERROR (problèmes serveur)
- **4xx** : Logged comme WARNING (erreurs client)

### 3. CORS Configurable (`cors.rs`)

Configuration adaptative selon l'environnement :

**Développement (`CORS_ORIGIN="*"`) :**
- Tous les origins autorisés
- Toutes les méthodes (GET, POST, PUT, DELETE, etc.)
- Tous les headers

**Production (origin spécifique) :**
```env
CORS_ORIGIN=https://pubg-tracker.azurewebsites.net
```
- Origin restreint à l'URL de production
- Méthodes autorisées : GET, POST, PUT, DELETE, PATCH
- Headers autorisés : Content-Type, Authorization, Accept

### 4. Instrumentation des Services

Tous les services clés sont instrumentés avec `#[tracing::instrument]` :

#### PubgApiService
- Logs des requêtes API PUBG
- Détection des rate limits
- Gestion des erreurs réseau
- Retry avec backoff exponentiel

**Exemple de logs :**
```json
{
  "level": "DEBUG",
  "message": "Requesting player data from PUBG API",
  "span": {
    "name": "get_player_by_name",
    "shard": "steam",
    "player_name": "PlayerOne"
  }
}
```

#### StatsService
- Cache hits/misses (mémoire et MongoDB)
- Calcul des statistiques
- Invalidation du cache

**Exemple de logs :**
```json
{
  "level": "DEBUG",
  "message": "Stats found in memory cache",
  "span": {
    "name": "get_or_compute_stats",
    "player_id": "507f1f77bcf86cd799439011",
    "period": "7d",
    "mode": "solo",
    "shard": "steam"
  }
}
```

#### PlayerService
- Opérations CRUD sur les joueurs
- Synchronisation avec l'API PUBG
- Gestion du cache

## Configuration des Niveaux de Log

### Variables d'Environnement

```env
# Development - logs détaillés
RUST_LOG=debug

# Production - logs essentiels
RUST_LOG=info

# Debug spécifique à un module
RUST_LOG=pubg_tracker_api::services::pubg_api_service=debug,info
```

### Niveaux de Log Utilisés

1. **TRACE** : Non utilisé (trop verbeux)
2. **DEBUG** : Opérations internes, cache hits/misses, étapes de traitement
3. **INFO** : Événements normaux (player ajouté, API call réussi, etc.)
4. **WARN** : Situations anormales non critiques (rate limit, retry)
5. **ERROR** : Erreurs critiques (échec API, erreur DB)

## Intégration dans le Router

Les middlewares sont appliqués dans l'ordre suivant :

```rust
let app = Router::new()
    .route("/health", get(health_check))
    .nest("/api", api_routes)
    .with_state(app_state)
    .layer(axum::middleware::from_fn(trace_request))   // 1. Traçage HTTP
    .layer(axum::middleware::from_fn(handle_errors))   // 2. Gestion erreurs
    .layer(create_cors_layer(&cors_origin));           // 3. CORS
```

**Important :** Les layers sont appliqués de bas en haut. Le CORS est appliqué en premier, puis la gestion d'erreur, puis le traçage.

## Tests de l'Observabilité

### Tester le Logging

```bash
# Démarrer avec logs debug
RUST_LOG=debug cargo run

# Tester une requête
curl http://localhost:8080/api/players

# Observer les logs JSON
```

### Vérifier les Middlewares

**Test du traçage :**
```bash
curl -i http://localhost:8080/health
# Vérifier dans les logs : method="GET" uri="/health" status=200 duration_ms=X
```

**Test CORS :**
```bash
curl -i -H "Origin: http://localhost:3000" http://localhost:8080/api/players
# Vérifier header : Access-Control-Allow-Origin: http://localhost:3000
```

**Test gestion d'erreur :**
```bash
curl http://localhost:8080/api/players/invalid-id
# Vérifier log WARNING pour 4xx
```

## Prochaines Étapes (Phase 6)

### Azure Application Insights

Intégration future pour :
- Métriques de performance
- Distributed tracing
- Alertes automatiques
- Dashboards personnalisés

### Métriques Personnalisées

À ajouter :
- Temps de réponse API PUBG
- Taux de cache hits
- Nombre de players actifs
- Erreurs par endpoint

## Dépendances Ajoutées

```toml
# Cargo.toml
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json", "env-filter"] }
```

## Fichiers Modifiés

### Nouveaux Fichiers
- `backend/src/middleware/logging.rs` - Traçage HTTP
- `backend/src/middleware/error.rs` - Gestion d'erreur
- `backend/src/middleware/cors.rs` - CORS configurable
- `backend/src/middleware/mod.rs` - Module middleware

### Fichiers Modifiés
- `backend/src/main.rs` - Intégration middlewares + logging config
- `backend/src/services/player_service.rs` - Instrumentation #[tracing::instrument]
- `backend/src/services/pubg_api_service.rs` - Instrumentation + logs
- `backend/src/services/stats_service.rs` - Instrumentation + logs

## Résumé

✅ **Logging structuré JSON** pour Application Insights  
✅ **Traçage HTTP complet** (méthode, URI, status, durée)  
✅ **Gestion automatique des erreurs** par status code  
✅ **CORS adaptatif** dev/production  
✅ **Instrumentation complète** des services critiques  
✅ **Configuration flexible** via variables d'environnement  

La Phase 5 est terminée. Le backend dispose maintenant d'une observabilité complète, essentielle pour la production et le debugging.
