# Multi-stage build for optimal image size
# Stage 1: Build environment
FROM rust:1.75-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

# Copy source code
COPY . .

# Build the application
RUN cargo build --release

# Stage 2: Runtime environment
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create non-root user
RUN groupadd -r fubon && useradd -r -g fubon fubon

# Create app directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/r-fubon-neo /usr/local/bin/r-fubon-neo

# Change ownership to non-root user
RUN chown -R fubon:fubon /app

# Switch to non-root user
USER fubon

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD r-fubon-neo version || exit 1

# Set default command
ENTRYPOINT ["r-fubon-neo"]
CMD ["version"]