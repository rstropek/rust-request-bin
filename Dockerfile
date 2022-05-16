ARG BUILD_BASE=rust:alpine
FROM ${BUILD_BASE} AS builder-base
RUN apk add --no-cache musl-dev

FROM builder-base AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM builder AS tests
RUN cargo test --release

ARG PROD_BASE=alpine
FROM alpine AS production
WORKDIR /app
COPY --from=builder /app/target/release/acr-beyond-images ./
COPY --from=builder /app/templates/ ./templates/
COPY --from=builder /app/Rocket.toml ./
EXPOSE 80
CMD ["/app/acr-beyond-images"]
