FROM rust:1.85 AS builder

COPY . . 

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq5 &&  apt-get install make && rm -rf /var/lib/apt/lists/*


ENV RUST_LOG=info

EXPOSE 5000

CMD ["make","run-prod"]