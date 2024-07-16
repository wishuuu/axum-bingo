FROM rust:1.79.0 AS base
LABEL authors="Oskar"

WORKDIR /app

RUN apt update;
RUN apt install --yes --no-install-recommends build-essential pkg-config;

RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y \
        nodejs \
    && rm -rf /var/lib/apt/lists/* \

RUN rustup install stable;
RUN cargo install --force cargo-make;

FROM base AS build

COPY . .
RUN cargo make build;

FROM debian:bookworm as deploy

EXPOSE 8080

RUN apt update;
RUN apt install --yes --no-install-recommends bind9-dnsutils iputils-ping iproute2 curl ca-certificates htop;

WORKDIR /deploy

COPY --from=build /app/target/release/axum-bingo /deploy
COPY --from=build /app/dist /deploy/dist/

ENTRYPOINT ["/deploy/axum-bingo", "-p", "8080" , "-o", "0.0.0.0"]
