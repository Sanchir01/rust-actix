FROM rust:1.85 AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    bash \
    git \
    make \
    gcc \
    gettext \
    musl-dev

COPY . .

RUN cargo build --release

CMD ["cargo", "run-prod"]