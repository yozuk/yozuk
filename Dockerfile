FROM rust:1.60.0 as builder
RUN apt-get update && apt-get install -y --no-install-recommends cmake
WORKDIR /usr/src/app

# Use the old snapshot to enable caching.
RUN git clone https://github.com/yozuk/yozuk.git && \
    cd yozuk/zuk && \
    git checkout 2dc8f6879ead4a5b1a01f47e4af98756737c1f87 && \
    cargo build --release && \
    cd /usr/src/app && \
    mv yozuk/target . && \
    rm -rf yozuk

COPY . .
RUN cargo install --path zuk

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/zuk /usr/local/bin/zuk
ENV PORT 8080
CMD ["zuk", "--mode", "http-server", "--server-addr", "0.0.0.0:8080", "--cors-origin", "https://yozuk.com"]