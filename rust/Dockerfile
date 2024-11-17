ARG BUILDER_VERSION=1.0.0

# Configure base image with buildtime dependencies.
FROM econialabs/rust-builder:$BUILDER_VERSION AS base
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl=7.88* \
    && rm -rf /var/lib/apt/lists/*
