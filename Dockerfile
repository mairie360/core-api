# Define build arguments
ARG RUST_VERSION=1.85.0

# Stage 1: Builder
FROM rust:${RUST_VERSION}-slim-bookworm AS builder

# Set working directory
WORKDIR /usr/local/src/boilerplate

# Copy files
COPY . .

# Fetch dependencies
RUN cargo fetch --locked

# Build the application
RUN cargo build --release

# Stage 2: Final Image
FROM debian:bookworm-slim AS runtime

# Copy compiled binary from builder
COPY --from=builder --chown=root:root /usr/local/src/boilerplate/target/release/boilerplate /usr/local/bin/boilerplate

# Change the binary permissions
RUN chmod 755 /usr/local/bin/boilerplate

# Add non-root user and group
RUN groupadd boilerplate
RUN useradd -m -g boilerplate boilerplate
USER boilerplate

# Run the application
ENTRYPOINT [ "/usr/local/bin/boilerplate" ]
CMD [ "" ]