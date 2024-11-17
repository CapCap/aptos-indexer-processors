ARG BUILDER_VERSION=1.0.0

# Configure base image with buildtime dependencies.
FROM econialabs/rust-builder:$BUILDER_VERSION AS base
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential=12.9 \
    cmake=3.25* \
    curl=7.88* \
    clang=1:14* \
    pkg-config=1.8* \
    libdw-dev=0.188* \
    libpq-dev=15* \
    libssl-dev=3.0* \
    libjemalloc-dev=5.3* \
    lld=1:14* \
    && rm -rf /var/lib/apt/lists/*
