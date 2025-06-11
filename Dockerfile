# syntax=docker/dockerfile:1
ARG RUST_VERSION=1.86.0
ARG RUST_TAG=slim-bookworm
ARG APP_NAME=rsflow


# Stage 1: Base com dependências de sistema
FROM rust:${RUST_VERSION}-${RUST_TAG} AS base
WORKDIR /app
RUN apt-get update && apt-get install -y clang lld pkg-config libssl-dev git


# Stage 2: Planejamento (cache de dependências)
FROM base AS planner
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


# Stage 3: Cache das dependências
FROM base AS cacher
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json


# Stage 4: Build de produção
FROM base AS builder
ENV USER=web
ENV UID=1001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release


# Stage 5-1: Desenvolvimento com hot-reload
FROM base AS dev
RUN cargo install cargo-watch
WORKDIR /app
COPY . .
EXPOSE 3000
CMD ["cargo", "watch", "-x", "check", "-x", "run"]


# Stage 5-2: Produção final (distroless)
FROM gcr.io/distroless/cc-debian12
ARG APP_NAME
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder /app/target/release/${APP_NAME} /app/${APP_NAME}
WORKDIR /app
USER web:web
CMD ["./rsflow"]
