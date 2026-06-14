# Build Stage
FROM rust:1.75-slim AS builder

WORKDIR /usr/src/app

# Copy files
COPY Cargo.toml ./
# Create empty src/main.rs to build dependencies and cache them
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy actual source code and rebuild
COPY src ./src
# Update timestamp of main.rs to ensure cargo rebuilds it
RUN touch src/main.rs
RUN cargo build --release

# Run Stage
FROM debian:bookworm-slim

WORKDIR /usr/local/bin

# Copy compiled binary from builder stage
COPY --from=builder /usr/src/app/target/release/jiosaavn-api-rust ./jiosaavn-api

# Install certificates for HTTPS requests
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

EXPOSE 8787

CMD ["./jiosaavn-api"]
