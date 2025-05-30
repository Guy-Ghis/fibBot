
FROM rust:alpine AS build

WORKDIR /app

COPY . .

RUN apk add --no-cache musl-dev pkgconfig openssl-dev

RUN cargo build --release

# Debugging: List the contents of the target/release directory
RUN ls -l /app/target/release

FROM alpine:latest

WORKDIR /app

RUN apk add --no-cache libgcc

COPY --from=build /app/target/release/fibbot /app/fibbot

ENTRYPOINT [ "/app/fibbot" ]
