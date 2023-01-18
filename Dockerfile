FROM rust:1-slim as chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef as preper
COPY . .
RUN cargo chef prepare --recipe-path /app/recipe.json

FROM chef as builder
RUN apt-get update && apt-get install -y libssl-dev pkg-config
COPY --from=preper /app/recipe.json /app/recipe.json
RUN cargo chef cook --release --recipe-path /app/recipe.json
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get upgrade
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/dualis-api /
ENTRYPOINT [ "/dualis-api" ]
