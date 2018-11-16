# Dockerfile for Linux CI builds

FROM rust:1.28

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    cmake \
    gcc \
    libc6-dev \
    make \
    pkg-config \
    libssl-dev \
    xvfb \
    xterm
