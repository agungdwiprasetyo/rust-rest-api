# Stage 1
FROM rust:1.49.0 AS service_builder

WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get upgrade -y && apt-get install -y build-essential git clang llvm-dev libclang-dev libssl-dev pkg-config libpq-dev musl-tools brotli

RUN USER=root cargo new rust-rest-api
WORKDIR /usr/src/rust-rest-api
COPY Cargo.toml ./
RUN cargo build --release

ENV PKG_CONFIG_ALLOW_CROSS=1
ENV OPENSSL_INCLUDE_DIR="/usr/include/openssl"
ENV RUSTFLAGS="-C target-feature=+crt-static"
COPY src ./src
COPY .env .
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Stage 2
FROM alpine:latest  

COPY --from=service_builder /usr/local/cargo/bin/rust-rest-api .
COPY --from=service_builder /usr/src/rust-rest-api/.env .env
USER 1000

ENTRYPOINT ["./rust-rest-api"]
