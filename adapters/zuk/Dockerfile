FROM rust:latest as builder
RUN apt-get update && apt-get install -y cmake
WORKDIR /usr/src/app

# Use the old snapshot to enable caching.
RUN git clone https://github.com/yozuk/yozuk.git && \
    cd yozuk/adapters/zuk && \
    git checkout 625fcccb740760c0384603d20b403463b6f8f1eb && \
    cargo build --release && \
    cd /usr/src/app && \
    mv yozuk/target . && \
    rm -rf yozuk

COPY . .
WORKDIR /usr/src/app/adapters/zuk
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/zuk /usr/local/bin/zuk
ENV PORT 8080
CMD ["zuk", "--mode", "http-server", "--server-addr", "0.0.0.0:8080", "--cors-origin", "https://yozuk.com"]