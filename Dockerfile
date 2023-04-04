# syntax = docker/dockerfile:1.4

################################################################################
FROM rust:1.68 AS base

################################################################################

FROM base AS builder

RUN set -eux; \
    apt update; \
    apt upgrade -y; \
    apt install --no-install-recommends lld clang curl ca-certificates -y

WORKDIR /app
COPY src src
COPY Cargo.toml Cargo.lock ./
RUN --mount=type=cache,target=/root/.rustup \
    --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/root/.cargo/git \
	--mount=type=cache,target=/app/target \
	set -eux; \
	cargo build --release; \
    cp target/release/catscii .

################################################################################
FROM debian:bullseye-slim AS runtime

SHELL ["/bin/bash", "-c"]

RUN set -eux; \
	apt update; \
	apt upgrade -y; \
	apt install -y --no-install-recommends ca-certificates; \
	apt clean autoclean; \
	apt autoremove -y; \
	rm -rf /var/lib/{apt,dpkg,cache,log}/

WORKDIR /app
COPY --from=builder /app/catscii .

CMD ["/app/catscii"]