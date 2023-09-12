FROM rust:latest as builder
WORKDIR /app/src
RUN USER=root cargo new --bin ht
COPY Cargo.toml ./ht/

WORKDIR /app/src/ht
RUN cargo build --release

COPY ./ ./
RUN cargo build --release
 
FROM ubuntu:jammy
WORKDIR /app
RUN apt update \
    && apt install -y openssl ca-certificates \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

EXPOSE 80 443

COPY --from=builder /app/src/ht/target/release/ht /app/

CMD ["/app/ht"]