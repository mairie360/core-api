# Define build arguments
ARG RUST_VERSION=1.85.0

# Stage 1: Builder
FROM rust:${RUST_VERSION}-slim-bookworm AS builder

# Set working directory
WORKDIR /usr/src/boilerplate-api

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
    strip target/release/boilerplate-api

# Stage 2: Runtime
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates wget libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user and group
RUN groupadd --system boilerplate-api && useradd --no-log-init --system -g boilerplate-api boilerplate-api

# Copy the compiled binary
COPY --from=builder --chown=boilerplate-api:boilerplate-api /usr/src/boilerplate-api/target/release/boilerplate-api /usr/local/bin/boilerplate-api

# Set permissions
USER boilerplate-api

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/boilerplate-api"]
CMD []

# Expose the port
EXPOSE 3000