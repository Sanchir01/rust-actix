PHONY:
SILENT:

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

