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

