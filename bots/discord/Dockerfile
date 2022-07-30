FROM rust:latest as builder
RUN apt-get update && apt-get install -y cmake
WORKDIR /usr/src/app

RUN cargo install http-server
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates libssl1.1 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/yozuk-discord /usr/local/bin/yozuk-discord
COPY --from=builder /usr/local/cargo/bin/http-server /usr/local/bin/http-server
COPY ./start.sh .
RUN chmod +x ./start.sh
CMD ./start.sh