FROM rust:alpine AS builder-base
RUN apk add --no-cache musl-dev

FROM builder-base AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine
WORKDIR /app
COPY --from=builder /app/target/release/acr-beyond-images ./
COPY --from=builder /app/templates/ ./templates/
COPY --from=builder /app/Rocket.toml ./
EXPOSE 80
CMD ["/app/acr-beyond-images"]
