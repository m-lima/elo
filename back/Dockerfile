FROM docker.io/rust:1.79.0-bookworm as rust
WORKDIR /src

COPY Cargo.toml Cargo.lock .
RUN mkdir src && echo 'fn main() {}' > src/main.rs && cargo build --release && rm -rf src
COPY . .
RUN cargo build --release

FROM docker.io/debian:stable-20240701-slim
WORKDIR /opt/elo
COPY --from=rust /src/target/release/elo .
EXPOSE 80
ENV CLICOLOR_FORCE 1

ENTRYPOINT ["./elo"]
