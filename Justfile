default:
  just -l

build-docs:
  docker run --rm -v "$PWD/openapis:/spec" redocly/cli build-docs /spec/api.yml

db:
  docker-compose up -d postgres

psql:
  docker-compose exec postgres psql -U poker_user -d poker_db

api:
  docker-compose up -d api

run:
  docker-compose up -d

# ローカル開発用コマンド
dev:
  DATABASE_URL=postgres://poker_user:poker_password@localhost:6432/poker_db cargo watch -x run

dev-check:
  DATABASE_URL=postgres://poker_user:poker_password@localhost:6432/poker_db cargo watch -x check -x test -x run

migrate:
  cd migration && cargo run

# ローカル開発セットアップ（DB起動 + マイグレーション）
setup-dev:
  just db
  sleep 5
  just migrate

# ローカル開発開始（DB起動 + ホットリロード）
start-dev:
  just setup-dev
  just dev
