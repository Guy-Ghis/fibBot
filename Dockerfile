FROM rust:alpine AS build

WORKDIR /app

COPY . .

RUN apk add musl-dev

RUN cargo build --release --verbose

FROM alpine:latest

WORKDIR /app

RUN apk add libgcc

COPY --from=build /app/target/release/fibbot /app/fibbot

ENTRYPOINT [ "/app/fibbot" ]