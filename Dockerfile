# Build stage
FROM rust:alpine AS build

# Install dependencies
RUN apk add --no-cache musl-dev openssl-dev libgcc

# Set up environment for OpenSSL
ENV OPENSSL_STATIC=1
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include

# Add the musl target
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

# Copy the source code
COPY . .

# Build the project
RUN cargo build --release --target x86_64-unknown-linux-musl --verbose

# Runtime stage
FROM alpine:latest

WORKDIR /app

# Install runtime dependencies
RUN apk add --no-cache libgcc

# Copy the binary from the build stage
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/fibbot /app/fibbot

# Set the entrypoint
ENTRYPOINT [ "/app/fibbot" ]