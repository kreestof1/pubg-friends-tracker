
# Spécifications Techniques — PUBG Tracker (Rust + MongoDB) sur Azure

**Version** : 1.1  
**Date** : 2026-02-09 01:12

## 1. Architecture cible
- **API** : backend Rust (Tokio + Axum) exposant `/players`, `/matches` etc.  
- **Client HTTP** : `reqwest` avec headers par défaut `Authorization: Bearer <api-key>` et `Accept: application/vnd.api+json`. citeturn1search8turn1search10  
- **MongoDB** : stockage des joueurs/matches. Cible **Azure Cosmos DB – API pour MongoDB** (ou Atlas). Cosmos DB offre compatibilité wire‑protocol MongoDB ; vérifier les gaps avant usages avancés. citeturn9search87turn9search89  
- **Hébergement** : image Docker déployée sur **Azure Container Apps** (ACA) avec autoscaling ; alternative **App Service – Web App for Containers**. citeturn9search69turn9search81  
- **Secrets** : **Azure Key Vault** + **Managed Identity** (pas de secrets en clair). citeturn9search75turn9search57  
- **Observabilité** : **Azure Monitor / Log Analytics** pour logs système/console ACA et requêtes Kusto. citeturn9search63

## 2. Flux techniques clés (PUBG)
1. **Recherche joueur** : `GET /shards/{platform}/players?filter[playerNames]=…` → extraire `data[0].id` (accountId) + `attributes.name`. citeturn1search9  
2. **Rafraîchir** : respecter **10 req/min** ; lire `X-RateLimit-Remaining`/`Reset` et backoff en cas de `429`. Les endpoints **/matches** et **télémétrie** ne comptent pas dans la limite. citeturn1search30  
3. **Match & télémétrie** : `GET /shards/{platform}/matches/{matchId}` puis chercher l’asset `attributes.URL` → téléchargement sans clé. citeturn1search13

## 3. Schéma de données (Mongo)
**Collection `players`**  
Indexes : `{ account_id: 1 }` (unique), `{ name: 1, shard: 1 }`.  
Document :
```json
{
  "account_id": "account.steam.xxxx",
  "name": "string",
  "shard": "steam|xbox|psn|...",
  "created_at": 0,
  "last_refreshed_at": 0,
  "last_matches": ["matchId1", "matchId2"],
  "summary": {}
}
```

## 4. Endpoints (contrat détaillé)
- `POST /players` → body `{ name, shard }` ; upsert par `account_id`.  
- `GET /players?page&limit&q` → liste paginée.  
- `GET /players/:id` → lecture par `_id`.  
- `POST /players/:id/refresh` → met à jour `last_matches`.  
- `GET /players/:id/matches` → retour IDs + liens `/matches` + URL télémétrie.

## 5. Azure — Déploiement & Ops
### 5.1 Conteneurisation & registre
- Conteneur Docker (Rust) poussé sur **Azure Container Registry (ACR)** ; déploiement vers **Container Apps**. Guides & outils (CLI/VS Code) facilitent build/push/deploy. citeturn9search72

### 5.2 Exécution (compute)
- **Azure Container Apps** : service serverless pour conteneurs, scaling automatique, gestion des secrets/config, ingress. citeturn9search69  
- Quickstart & CLI : création d’un environnement ACA, déploiement d’une image, intégration Log Analytics. citeturn9search74

### 5.3 Base de données
- **Azure Cosmos DB – API pour MongoDB** : utilisation des **drivers MongoDB existants** avec connexion via la chaîne Mongo. Attention : certaines fonctionnalités MongoDB ne sont pas supportées (compat partielle selon version). citeturn9search87turn9search89

### 5.4 Secrets & identité
- **Azure Key Vault** pour stocker la clé PUBG et la chaîne Mongo ; recommandations de rotation et gestion des accès. citeturn9search75  
- **Managed Identity** de l’app pour lire Key Vault et, si configuré, s’authentifier auprès d’Azure services (remplace les secrets statiques). citeturn9search57

### 5.5 Observabilité
- **Logs ACA** : **Console** et **System** logs alimentent **Log Analytics** ; requêtes Kusto pour diagnostics/alertes. citeturn9search63  
- **Azure Monitor** / Container insights : requêtes et tableaux de bord sur les logs conteneurs. citeturn9search67

## 6. Sécurité & Réseau
- RBAC Azure + principe du moindre privilège. citeturn9search58  
- Secretless : Key Vault + Managed Identity ; **aucune clé PUBG** en clair dans le code. citeturn9search75turn9search57  
- Optionnel : endpoints privés/pare‑feu pour Cosmos DB (Mongo) et Key Vault.

## 7. CI/CD (exemple)
- **GitHub Actions** : build Docker → push ACR → déploiement ACA ; variables: `REGISTRY_LOGIN_SERVER`, `AZURE_CREDENTIALS` (OIDC recommandé) ; secrets applicatifs en Key Vault.

## 8. Exigences de performance & tests
- **Timeout** `reqwest` 15s ; retries expo pour 429/5xx PUBG. citeturn1search30  
- Tests unitaires (parsing, mapping), intégration (Mongo + API simulée), charge (p95 < 1s hors appels externes).

## 9. Risques & mitigations
- **Quota PUBG (10 RPM)** : cache, queue de rafraîchissement, priorisation par utilisateur. citeturn1search30  
- **Compat Cosmos/Mongo** : vérifier besoins (indexation texte, stages d’agg) avant d’activer ; sinon **MongoDB Atlas sur Azure**. citeturn9search89

## 11 bis. Dashboard comparatif (technique)
- **Objectif** : fournir des agrégations prêtes à afficher pour comparer plusieurs joueurs de manière fluide (UX claire et ludique).
- **Endpoints** (proposés) :
  - `GET /dashboard?ids=<id1,id2,...>&period=last30d&mode=squad&shard=steam` → retourne, par joueur, les métriques agrégées (kills, K/D, win rate, dégâts, temps de survie, top‑1...).
  - `GET /players/:id/stats?period=...` → détail par joueur pour nourrir les tooltips/mini‑cartes.
- **Agrégations** : privilégier des pipelines simples compatibles avec **Cosmos DB API pour MongoDB** (ex. `$match`, `$group`, `$project`) ; si une étape avancée manque, **calculer côté service** (Rust) pour rester portable. citeturn9search87turn9search89
- **Caching** :
  - Cache mémoire (TTL 60–300 s) **et/ou** collection `stats_cache` avec TTL pour soulager l’API PUBG et la base.
  - Invalidation ciblée lors d’un `POST /players/:id/refresh`.
- **UX** : livrer des structures JSON génériques (séries, labels) ; le front choisit la visualisation (barres, radar, leaderboard) ; micro‑interactions légères pour garder une expérience « claire et amusante ».
- **Accessibilité & responsive** : contrastes AA, focus visibles, navigation clavier ; grille responsive ≥ 320 px.

## 12. Annexes (références clés)
- App Service/Containers & docs Linux. citeturn9search81  
- Container Apps — concepts & déploiement. citeturn9search69turn9search74  
- Cosmos DB — API MongoDB (overview). citeturn9search87  
- Key Vault — sécuriser les secrets. citeturn9search75  
- Managed Identity — overview. citeturn9search57  
- PUBG — auth, headers, endpoints, télémétrie, rate limits. citeturn1search8turn1search10turn1search9turn1search13turn1search30
