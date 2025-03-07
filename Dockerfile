# Define build arguments
ARG RUST_VERSION=1.85.0

# Stage 1: Builder
FROM rust:${RUST_VERSION}-slim-bookworm AS builder

# Set working directory
WORKDIR /usr/src/boilerplate

# Install dependencies for building
RUN apt-get update && apt-get install -y --no-install-recommends \
    binutils \
    && rm -rf /var/lib/apt/lists/*

# Copy the source code
COPY . .

# Fetch dependencies
RUN cargo fetch --locked

# Build the application
RUN cargo build --release --locked && \
    strip target/release/boilerplate

# Stage 2: Runtime
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates wget \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user and group
RUN groupadd --system boilerplate && useradd --no-log-init --system -g boilerplate boilerplate

# Copy the compiled binary
COPY --from=builder --chown=boilerplate:boilerplate /usr/src/boilerplate/target/release/boilerplate /usr/local/bin/boilerplate

# Set permissions
USER boilerplate

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/boilerplate"]
CMD []