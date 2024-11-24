# Build stage
FROM rust:1.75-slim as builder

WORKDIR /usr/src/app
COPY . .

# Build dependencies separately to cache them
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/rust-flagsmith .

# Set the binary as the entrypoint
ENTRYPOINT ["./rust-flagsmith"]
