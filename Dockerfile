FROM rust:1.83-slim
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release