# CHEF
FROM lukemathwalker/cargo-chef:latest-rust-1.80-alpine AS chef
WORKDIR /app

RUN apk upgrade --no-cache --update
RUN apk add --no-cache musl-dev

# PLANNER
FROM chef AS planner

COPY --chmod=500 --chown=runner:runner ./Cargo.* ./
COPY --chmod=500 --chown=runner:runner ./src ./src
RUN cargo chef prepare --recipe-path ./recipe.json

# BUILDER
FROM chef AS builder

COPY --chmod=500 --from=planner /app/recipe.json ./recipe.json
RUN cargo chef cook --release --recipe-path ./recipe.json

COPY --chmod=500 --chown=runner:runner ./Cargo.* ./
COPY --chmod=500 --chown=runner:runner ./src ./src
RUN cargo build --release

# RUNNER
FROM rust:1.80-alpine AS runner
WORKDIR /app

RUN apk upgrade --no-cache --update

RUN adduser --disabled-password runner
RUN chown runner -R /app

USER runner

COPY --chmod=500 --chown=runner:runner --from=builder /app/target/release ./
ENTRYPOINT [ "/app/poliwarden" ]
