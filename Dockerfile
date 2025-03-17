# Define build arguments
ARG RUST_VERSION=1.85.0

# Stage 1: Builder
FROM rust:${RUST_VERSION}-slim-bookworm AS builder

# Set working directory
WORKDIR /usr/src/core-api

# Install dependencies for building
RUN apt-get update && apt-get install -y --no-install-recommends \
    binutils libpq-dev \    
    && rm -rf /var/lib/apt/lists/*

# Copy the source code
COPY . .

# Fetch dependencies
RUN cargo fetch --locked

# Build the application
RUN cargo build --release --locked && \
    strip target/release/core-api

# Stage 2: Runtime
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates wget libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user and group
RUN groupadd --system core-api && useradd --no-log-init --system -g core-api core-api

# Copy the compiled binary
COPY --from=builder --chown=core-api:core-api /usr/src/core-api/target/release/core-api /usr/local/bin/core-api

# Set permissions
USER core-api

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/core-api"]
CMD []
