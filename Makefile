PHONY:
SILENT:
MIGRATION_NAME ?= new_migration

PASSWORD ?= password
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

docker-build:
	docker build -t . actix-rest

docker: docker-build
	docker-compose up

compose:
	docker-compose up

compile:
	rustc --edition 2024 src/main.rs
migrations-up:
	goose -dir migrations postgres "host=localhost user=postgres password=avito port=5432 dbname=postgres sslmode=disable"  up

migrations-down:
	goose -dir migrations postgres  "host=localhost user=postgres password=avito port=5432 dbname=postgres sslmode=disable"  down

migrations-status:
	goose -dir migrations postgres  "host=localhost user=postgres password=avito port=5432 dbname=postgres sslmode=disable" status

migrations-new:
	goose -dir migrations create $(MIGRATION_NAME) sql

