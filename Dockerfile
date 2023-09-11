# Start from the latest Rust base image
FROM rust:latest as builder

# Set the current working directory inside the container
WORKDIR /usr/src

# Copy over your manifest
COPY ./Cargo.toml ./Cargo.toml

# This build step will cache your dependencies
RUN mkdir src \
    && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs \
    && cargo build --release \
    && rm -rf src/

# Copy your source tree
COPY ./src ./src

# Build for release. 
RUN cargo build --release && ls /usr/src/target/release/

# Our second stage, that creates the final image
FROM ubuntu:jammy

# Copy the build artifact from the builder stage and create a new binary.
COPY --from=builder /usr/src/target/release/integra_project /usr/local/bin

# Set the binary as the entrypoint of the container
ENTRYPOINT ["integra_project"]