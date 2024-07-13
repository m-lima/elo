# Build
FROM docker.io/rust:1.79.0-bookworm as rust
WORKDIR /src

COPY Cargo.toml Cargo.lock .
RUN mkdir src && echo 'fn main() {}' > src/main.rs && cargo build --release && rm -rf src
COPY . .
RUN touch src/main.rs && cargo build --release

# Deploy
FROM docker.io/debian:stable-20240701-slim
WORKDIR /opt/elo

RUN apt update && apt install -y libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=rust /src/target/release/elo .
EXPOSE 80
ENV CLICOLOR_FORCE 1

ENTRYPOINT ["./elo"]
