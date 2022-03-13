FROM rust:latest as builder
RUN apt-get update && apt-get install -y cmake
WORKDIR /usr/src/app

# Use the old snapshot to enable caching.
RUN git clone https://github.com/yozuk/yozuk.git && \
    cd yozuk/adapters/telegram && \
    git checkout 5558c53cb6ae4bb47b38980ca75bd32c07c4eb8d && \
    cargo build --release && \
    cd /usr/src/app && \
    mv yozuk/target . && \
    rm -rf yozuk

COPY . .
WORKDIR /usr/src/app/adapters/telegram
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates libssl1.1 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/yozuk-telegram /usr/local/bin/yozuk-telegram
ENV PORT 8080
CMD ["yozuk-telegram"]