PHONY:
SILENT:
MIGRATION_NAME ?= new_migration

LOCAL_DB_HOST=localhost
LOCAL_DB_PORT=5433
LOCAL_DB_USER=postgres
LOCAL_DB_PASSWORD=postgres
LOCAL_DB_NAME=rust-axum
LOCAL_DB_SSLMODE=disable

# Переменные для подключения к удаленной базе данных
REMOTE_DB_HOST=82.97.253.80
REMOTE_DB_PORT=5436
REMOTE_DB_USER=postgres
REMOTE_DB_PASSWORD=vrTboWaFAYwGsanAdxCwWIEkXMo1DGlCWLM6ZP18t2wFCb2AIccp31fjqAUOpSlv
REMOTE_DB_NAME=postgres
REMOTE_DB_SSLMODE=disable

LOCAL_DB_URL=postgres://$(LOCAL_DB_USER):$(LOCAL_DB_PASSWORD)@$(LOCAL_DB_HOST):$(LOCAL_DB_PORT)/$(LOCAL_DB_NAME)?sslmode=$(LOCAL_DB_SSLMODE)
REMOTE_DB_URL=postgres://$(REMOTE_DB_USER):$(REMOTE_DB_PASSWORD)@$(REMOTE_DB_HOST):$(REMOTE_DB_PORT)/$(REMOTE_DB_NAME)?sslmode=$(REMOTE_DB_SSLMODE)


run:
	cargo run

dev:
	cargo watch -w src -x run

build:
	cargo build

update:
	cargo update

build-prod:
	cargo build --release

run-prod:
	cargo run --release

docker-build:
	docker build -t  rust-axum .

docker: docker-build
	docker-compose up

compose:
	docker-compose up

migrations-up-local:
	goose -dir migrations postgres "$(LOCAL_DB_URL)" up

# Задача для отката миграций на локальной базе данных
migrations-down-local:
	goose -dir migrations postgres "$(LOCAL_DB_URL)" down

# Задача для проверки статуса миграций на локальной базе данных
migrations-status-local:
	goose -dir migrations postgres "$(LOCAL_DB_URL)" status

# Задача для применения миграций на удаленной базе данных
migrations-up-remote:
	goose -dir migrations postgres "$(REMOTE_DB_URL)" up

# Задача для отката миграций на удаленной базе данных
migrations-down-remote:
	goose -dir migrations postgres "$(REMOTE_DB_URL)" down

# Задача для проверки статуса миграций на удаленной базе данных
migrations-status-remote:
	goose -dir migrations postgres "$(REMOTE_DB_URL)" status

migrations-new:
	goose -dir migrations create $(MIGRATION_NAME) sql

