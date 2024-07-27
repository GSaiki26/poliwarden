# Args
ARG RUST_VERSION=1.80
ARG ALPINE_VERSION=latest

# CHEF
FROM lukemathwalker/cargo-chef:latest-rust-${RUST_VERSION}-alpine AS chef
WORKDIR /app

RUN apk upgrade --no-cache --update
RUN apk add --no-cache musl-dev

# PLANNER
FROM chef AS planner

COPY --chmod=500 ./Cargo.* ./
COPY --chmod=500 ./src ./src
RUN cargo chef prepare --recipe-path ./recipe.json

# BUILDER
FROM chef AS builder

COPY --chmod=500 ./Cargo.* ./
COPY --chmod=500 --from=planner /app/recipe.json ./recipe.json
RUN cargo chef cook --release --recipe-path ./recipe.json

ARG FEATURES
COPY --chmod=500 ./src ./src
RUN cargo build --release -F "${FEATURES}"

# RUNNER
FROM alpine:${ALPINE_VERSION} AS runner
WORKDIR /app

RUN apk upgrade --no-cache --update

RUN addgroup -g 9999 runner && adduser -G runner -u 9999 -D runner
RUN chown runner -R /app

USER runner

COPY --chmod=500 --chown=runner:runner --from=builder /app/target/release ./
ENTRYPOINT [ "/app/poliwarden" ]
