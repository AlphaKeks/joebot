FROM lukemathwalker/cargo-chef:latest-rust-1.71.0 AS chef
WORKDIR /joe


FROM chef AS planner
COPY src ./src
COPY Cargo.toml .
COPY Cargo.lock .
COPY config.docker.toml .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
COPY --from=planner /joe/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY src ./src
COPY Cargo.toml .
COPY Cargo.lock .
COPY config.docker.toml .
RUN cargo build --release


FROM debian:bullseye-slim AS runtime
WORKDIR /
COPY --from=builder /joe/config.docker.toml /etc/joe.toml
COPY --from=builder /joe/target/release/joebot /usr/local/bin/joebot
ENV RUST_LOG=WARN,joebot=TRACE
ENTRYPOINT ["/usr/local/bin/joebot", "--debug", "--config", "/etc/joe.toml"]
