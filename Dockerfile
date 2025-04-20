# -------- Stage 1: Build Static Binary --------
FROM rustlang/rust:nightly-slim AS builder

# Install MUSL tools for static linking
RUN apt-get update && apt-get install -y musl-tools

# Add MUSL target to enable static build
RUN rustup target add x86_64-unknown-linux-musl

# Set working directory inside the build container
WORKDIR /app

# Copy the entire project source into the container
COPY . .

# Build the binary statically using MUSL
RUN cargo build --release --target x86_64-unknown-linux-musl

# -------- Stage 2: Runtime Container --------
FROM alpine:latest

# Set working directory in the minimal runtime container
WORKDIR /app

# Copy the statically built binary from the builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/plugin-orchestrator .

# Optional: copy environment file if needed at runtime
COPY .env .env

# Expose the server port
EXPOSE 4000

# Run the binary
CMD ["./plugin-orchestrator"]
