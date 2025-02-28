# Stage 1: Build
FROM rust:alpine AS build

# Install necessary dependencies
RUN apk add --no-cache musl-dev pkgconfig openssl-dev

WORKDIR /app

COPY . .

# Build the release binary
RUN cargo build --release

# Stage 2: Runtime
FROM alpine:latest

WORKDIR /app

# Install runtime dependencies (libgcc is often needed for Rust binaries on Alpine)
RUN apk add --no-cache libgcc

# Copy the compiled binary from the build stage
COPY --from=build /app/target/release/fibbot /app/fibbot

# Set the entrypoint
ENTRYPOINT [ "/app/fibbot" ]