# FROM clux/muslrust:1.66.1-stable as chef
# USER root
# RUN cargo install cargo-chef
# RUN apt-get update && apt-get -y install python3-pip ca-certificates cmake libssl-dev && rm -rf /var/lib/apt/lists/*
# WORKDIR /app

# FROM chef as planner
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json

# FROM chef as builder

# COPY --from=planner /app/recipe.json recipe.json
# RUN cargo chef cook --target x86_64-unknown-linux-musl --release --recipe-path recipe.json
# COPY . .
# RUN cargo build --target x86_64-unknown-linux-musl --release

# FROM alpine:3 as app
# RUN apk --no-cache add ca-certificates
# COPY --from=builder /target/x86_64-unknown-linux-musl/release/dualis-api .
# CMD [ "/dualis-api" ]

# FROM ghcr.io/cross-rs/aarch64-unknown-linux-musl as base
# USER root

# # FROM base as planner
# # COPY . .
# # RUN cargo chef prepare --recipe-path recipe.json

# FROM base as builder
# # COPY --from=planner /app/recipe.json recipe.json
# # RUN cargo chef cook --target aarch64-unknown-linux-musl --release --recipe-path recipe.json
# COPY . .
# # RUN cargo build --target aarch64-unknown-linux-musl --release
# RUN cross build --target aarch64-unknown-linux-musl --release

# FROM rust:1 as chef
# USER root
# RUN cargo install cargo-chef
# WORKDIR /app

# FROM chef as preper
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json

# FROM chef as builder
# RUN apt-get update && apt-get install -y \
#   musl-dev \
#   musl-tools \
#   file \
#   git \
#   openssh-client \
#   make \
#   cmake \
#   g++ \
#   curl \
#   pkgconf \
#   ca-certificates \
#   xutils-dev \
#   libssl-dev \
#   libpq-dev \
#   automake \
#   autoconf \
#   libtool \
#   protobuf-compiler \
#   libprotobuf-dev \
#   --no-install-recommends && \
#   rm -rf /var/lib/apt/lists/*
# RUN rustup target add aarch64-unknown-linux-musl
# ENV SSL_VER="1.1.1q" \
#     PREFIX="/usr" \
#     LD_LIBRARY_PATH=$PREFIX \
#     PKG_CONFIG_PATH="/usr/local/lib/pkgconfig"
# RUN curl -sSL https://www.openssl.org/source/openssl-$SSL_VER.tar.gz | tar xz && \
#     cd openssl-$SSL_VER && \
#     ./Configure no-zlib no-shared -fPIC --prefix=$PREFIX --openssldir=$PREFIX/ssl linux-aarch64 -DOPENSSL_NO_SECURE_MEMORY && \
#     env C_INCLUDE_PATH=$PREFIX/include make depend 2> /dev/null && \
#     make -j$(nproc) && make install && \
#     cd .. && rm -rf openssl-$SSL_VER
# ENV PKG_CONFIG_ALLOW_CROSS=true \
#     PKG_CONFIG_ALL_STATIC=true \    
#     CC="musl-gcc" \
#     LD="musl-ldd" \
#     OPENSSL_STATIC=true \
#     OPENSSL_DIR=$PREFIX \
#     PKG_CONFIG_PATH=$PREFIX/lib/pkgconfig

# COPY --from=preper /app/recipe.json /app/recipe.json
# RUN cargo chef cook --release --target aarch64-unknown-linux-musl --recipe-path /app/recipe.json
# COPY . .
# RUN cargo build --release --target aarch64-unknown-linux-musl

# FROM rust:1-alpine3.17 as chef
FROM rust:1-slim as chef
USER root
# RUN apk update && apk upgrade
# RUN apk update && apk add musl-dev
RUN cargo install cargo-chef
WORKDIR /app

FROM chef as preper
COPY . .
RUN cargo chef prepare --recipe-path /app/recipe.json

FROM chef as builder
# RUN apk update && apk add openssl-dev ca-certificates
RUN apt-get update && apt-get install -y libssl-dev pkg-config
COPY --from=preper /app/recipe.json /app/recipe.json
RUN cargo chef cook --release --recipe-path /app/recipe.json
COPY . .
RUN cargo build --release

# FROM alpine:3.17 as app
FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y ca-certificates
RUN apt-get update && apt-get upgrade
COPY --from=builder /app/target/release/dualis-api /
ENTRYPOINT [ "/dualis-api" ]