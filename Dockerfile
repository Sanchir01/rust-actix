FROM ubuntu:22.04 AS builder

RUN apt-get update && apt-get install -y \
    curl build-essential gcc g++ \
    libssl-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

COPY . .

RUN cargo build --release

FROM ubuntu:22.04 AS runtime

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/my_rust_app /usr/local/bin/my_rust_app


WORKDIR /usr/local/bin


CMD ["./my_rust_app"]