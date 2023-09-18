# Yggdrasil

Backend for WiBruhTor service

## Commands

### Run development

#### Run development environment

```bash
docker compose -f docker-compose.development.yml up
```

#### Run sqlx

Before this create `.env` file using `.env.example`

```bash
sqlx database create && sqlx migrate run
```

#### Run with watch

Before this create `.env` file using `.env.example`

```bash
cargo watch -q -c -w src/ -x run
```

### Run production

Before this create `.env` file using `.env.example`

```bash
docker compose -f docker-compose.yml up -d
```

### Before commit

```bash
cargo sqlx prepare
```