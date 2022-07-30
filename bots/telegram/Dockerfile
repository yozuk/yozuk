FROM rust:latest as builder
RUN apt-get update && apt-get install -y cmake
WORKDIR /usr/src/app

COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates libssl1.1 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/yozuk-telegram /usr/local/bin/yozuk-telegram
ENV PORT 8080
CMD ["yozuk-telegram"]