FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM alpine:3.20

RUN apk add --no-cache ca-certificates tzdata \
    && adduser -D -u 1000 appuser

USER appuser
WORKDIR /home/appuser

COPY --from=builder /app/target/release/editorial_tui /usr/local/bin/editorial_tui

ENV CORE_API_URL=http://10.20.30.10:8080
ENV MOCK_MODE=true

ENTRYPOINT ["editorial_tui"]
