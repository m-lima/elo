FROM docker.io/rust:1.79.0-bookworm as rust
WORKDIR /src

COPY . .
RUN cargo build --release

FROM docker.io/debian:stable-20240701-slim
WORKDIR /opt/elo
COPY --from=rust /src/target/release/elo .
EXPOSE 80
ENV CLICOLOR_FORCE 1

ENTRYPOINT ["./elo"]
