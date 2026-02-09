
# Spécifications Fonctionnelles — PUBG Tracker (Rust + MongoDB) sur Azure

**Version** : 1.1  
**Date** : 2026-02-09 01:12

## 1. Contexte & Objectifs
Construire une application web permettant de **récupérer, stocker et consulter** les informations des joueurs **PUBG** déjà enregistrés. L’application s’appuie sur l’**API officielle PUBG** pour l’acquisition des données, un backend **Rust** et une base **MongoDB**. Le déploiement cible **Microsoft Azure** pour bénéficier d’une plateforme managée, de la sécurité (Key Vault, Managed Identity) et de l’observabilité (Monitor/Log Analytics).

Références essentielles :
- Authentification et en-têtes de l’API PUBG (`Authorization: Bearer <api-key>`, `Accept: application/vnd.api+json`) ; recherche joueurs via `/shards/{platform}/players?filter[playerNames]` ; **rate limit par défaut : 10 req/min** ; endpoints `/matches` et **télémétrie** non limités. citeturn1search8turn1search10turn1search9turn1search30
- La télémétrie est récupérée en lisant le match puis l’asset `attributes.URL` et se télécharge **sans clé**. citeturn1search13
- Hébergement applicatif managé sur Azure (App Service / Container Apps), observabilité via Log Analytics. citeturn9search81turn9search63

## 2. Périmètre (MVP)
- **Accueil (Dashboard comparatif)** : la page d’**accueil** de l’application est un **dashboard** permettant de **comparer les statistiques** des joueurs de façon **claire et amusante** (kills, K/D, win rate, dégâts, temps de survie, top‑1, etc.), avec filtres (période, shard, mode de jeu) et visualisations (barres, radar, leaderboard).  
- **Plateformes** : `steam`, `xbox`, `psn` (extensible `kakao`, `stadia`). citeturn1search27
- **Données cibles** : fiche joueur (accountId, nom, shard), liste des matches récents (IDs), détails de match et **URL de télémétrie**.
- **Actions utilisateur** :
  1. Ajouter un joueur (nom + shard) → résolution `accountId` et enregistrement.
  2. Lister les joueurs enregistrés (recherche/tri/pagination).
  3. Consulter un joueur (infos + matches récents).
  4. Rafraîchir un joueur (met à jour `last_matches`).
  5. Lister les matches d’un joueur (avec lien vers match et URL télémétrie).

## 3. Personae & Cas d’usage
- **Analyste/Coach** : souhaite récupérer rapidement les derniers matchs d’un joueur et le lien de télémétrie pour analyse.  
- **Community Manager** : suit un pool de joueurs et vérifie l’activité récente.  
- **Admin/Opérations** : déploie, supervise et sécurise l’application sur Azure (alerts/logs/secret management).

## 4. Parcours Utilisateur
0. **Accueil — Dashboard comparatif** : au chargement, l’utilisateur voit un **dashboard** synthétique et ludique permettant de **comparer** rapidement les joueurs enregistrés, d’appliquer des **filtres** (période, shard, mode) et d’afficher des **graphiques** (barres/radar/leaderboard).  
1. **Ajouter un joueur** : formulaire *Nom/Shard* → appel PUBG `/players?filter[playerNames]` → enregistrement. citeturn1search9  
2. **Lister** → recherche/tri sur `name`, `shard`, `last_refreshed_at`.  
3. **Voir** → affiche `account_id`, `shard`, `last_matches`.  
4. **Rafraîchir** → réinterroge `/players` (respect du quota) → met à jour `last_matches`. citeturn1search30  
5. **Matches** → pour chaque ID, lien `/matches/{id}` + **URL télémétrie** si présente. citeturn1search13  

## 5. Règles Métier & Contraintes
- **Clé API PUBG** : stockée côté serveur (Azure Key Vault), jamais exposée au navigateur. citeturn9search75
- **Rate limit** : 10 req/min ; backoff si `429` selon `X-RateLimit-Reset`. citeturn1search30
- **Cache/Persistance** : éviter les lectures redondantes ; persister joueur/matches en MongoDB.
- **Sécurité cloud** : usage des **Managed Identities** pour accéder aux ressources Azure sans secret statique. citeturn9search57

## 6. Données (Modèle fonctionnel)
**Player**  
- `account_id` (string, unique), `name` (string), `shard` (string), `created_at` (epoch), `last_refreshed_at` (epoch?), `last_matches` (string[]), `summary` (json?).

## 7. API interne (contrat haut niveau)
- `POST /players` → ajoute ou associe un joueur.  
- `GET /players` → liste (pagination via `page`,`limit`).  
- `GET /players/:id` → fiche.  
- `POST /players/:id/refresh` → rafraîchit `last_matches`.  
- `GET /players/:id/matches` → IDs + liens match + URL télémétrie.

## 8. Sécurité & Conformité
- Secrets (clé PUBG, chaînes de connexion) dans **Azure Key Vault** ; rotation périodique ; pas de secrets en dur. citeturn9search75  
- **Managed Identity** pour l’accès Key Vault / Cosmos DB (Mongo API). citeturn9search57  
- RBAC Azure pour l’administration (principe du moindre privilège). citeturn9search58

## 9. Hébergement & Observabilité (Azure)
- **Azure Container Apps** (ou App Service) pour exécuter le conteneur Rust ; intégration **Log Analytics** pour logs/metrics. citeturn9search69turn9search63  
- **Base** : Azure Cosmos DB **API pour MongoDB** (ou MongoDB Atlas sur Azure). Cosmos expose un protocole MongoDB compatible, mais attention aux écarts de compatibilité selon versions/fonctionnalités. citeturn9search87turn9search89

## 10. Non‑fonctionnels (SLA & Qualité)
- **Perf** : p50 < 500 ms hors appels externes.  
- **Disponibilité** : dépend des SLA de Container Apps/App Service et Cosmos DB.  
- **Observabilité** : requêtes Kusto (Log Analytics) ; alertes sur taux d’erreur/API PUBG et latence. citeturn9search63

## 11. Critères d’acceptation (MVP)
- **Accueil/Dashboard** :
  - Affiche au moins **5 métriques** par joueur (ex. K/D, kills, win rate, dégâts, temps de survie).  
  - **Comparer** au moins **2 joueurs** simultanément.  
  - **Filtres** actifs : période (7/30/90 jours), shard, mode (solo/duo/squad).  
  - **Visualisations** : au moins 2 types (barres **et** leaderboard), interactions fluides (<120ms).  
- **Ajout joueur** : si PUBG renvoie 200 + `id`, le joueur est **upsert** en base et retourné ; sinon 404 si vide. citeturn1search9  
- **Rafraîchir** : si `429`, l’API attend le reset et réessaie (≤2 fois). citeturn1search30  
- **Télémétrie** : l’URL est bien extraite depuis le match `included.asset.attributes.URL`. citeturn1search13  
- **Sécurité** : aucune fuite de la clé PUBG ; secrets servés depuis Key Vault ; accès applicatif via Managed Identity. citeturn9search75turn9search57
