# Template Axum SQLx API

Ce projet est un template pour une API REST construite avec Rust, Axum et SQLx. Il inclut une configuration de base pour le logging, la gestion des erreurs, et l'intégration avec une base de données PostgreSQL.

## Fonctionnalités

- 🚀 API REST avec Axum
- 🗄️ Intégration avec PostgreSQL via SQLx
- 📝 Logging structuré avec tracing
- 🔄 Gestion des erreurs avec thiserror
- 📚 Documentation OpenAPI
- 🧪 Tests d'intégration avec une base de données de test

## Prérequis

- Rust (dernière version stable)
- Cargo (>= 1.75.0)
- Git (>= 2.42.0)
- PostgreSQL
- Sqlx Client (cargo install sqlx-cli)

## Installation

1. Cloner le repository :
```bash
git clone {link}
cd template-axum-sqlx-api
```

2. Configurer la base de données :
   - Créer une base de données PostgreSQL
   - Copier `config.toml.example` vers `config.toml`
   - Modifier les paramètres de connexion dans `config.toml`

## Développement

### Base de données de développement

Un fichier `compose.yml` est fourni dans le dossier `assets/` pour lancer rapidement une base de données PostgreSQL de développement :

```bash
cd assets
docker compose up -d
```

La base de données sera accessible sur `localhost:5432` avec les identifiants par défaut :
- Utilisateur : `postgres`
- Mot de passe : `postgres`
- Base de données : `template_db`

### Lancer l'application

```bash
cargo run
```

L'API sera disponible sur `http://localhost:3000`.

### Tests

Pour les tests d'intégration, un fichier `compose.yml` est fourni pour lancer une base de données PostgreSQL de test :

```bash
docker compose up -d
```

Puis lancer les tests :

```bash
cargo test
```

### Documentation

La documentation OpenAPI est disponible à `http://localhost:3000/api/swagger`.

## Structure du projet

```
.
├── src/
│   ├── config.rs      # Configuration de l'application
│   ├── error.rs       # Gestion des erreurs
│   ├── handlers/      # Gestionnaires de routes
│   ├── models/        # Modèles de données
│   └── main.rs        # Point d'entrée
├── tests/             # Tests d'intégration
├── assets/           # Ressources (compose.yml, etc.)
├── config.toml        # Configuration
└── Cargo.toml         # Dépendances
```

## Contribution

1. Fork le projet
2. Créer une branche pour votre fonctionnalité
3. Commiter vos changements
4. Pousser vers la branche
5. Ouvrir une Pull Request

## Licence

MIT 