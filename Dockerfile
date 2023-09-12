FROM rust:latest as builder
WORKDIR /usr/src
RUN USER=root cargo new --bin integra_project
COPY Cargo.toml ./integra_project/

WORKDIR /usr/src/integra_project
RUN cargo build --release

COPY ./ ./
RUN cargo build --release
 
FROM ubuntu:jammy
WORKDIR /app
RUN apt update \
    && apt install -y openssl ca-certificates \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

EXPOSE 3000

COPY --from=builder /usr/src/integra_project/target/release/integra_project /app/

CMD ["/app/integra_project"]