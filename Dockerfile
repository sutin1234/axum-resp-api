FROM rust:1.68.2-slim-buster as backend

RUN apt update && apt install -y --no-install-recommends clang llvm perl
RUN apt install -y musl-tools musl-dev pkg-config
RUN apt install -y openssl libssl-dev
RUN update-ca-certificates
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /app/backend

COPY . /app/backend
ENV CC_x86_64_unknown_linux_musl=clang
ENV RUST_BACKTRACE=full
RUN cargo clean && cargo build --target x86_64-unknown-linux-musl --release

# RUN \
# --mount=type=cache,target=/app/backend/target,rw \
# --mount=type=cache,target=/usr/local/cargo/registry,rw \
# cd /app/backend && \
# cargo build --target x86_64-unknown-linux-musl --release
# cp /app/backend/target/x86_64-unknown-linux-musl/release/backend /app/backend/server

# FROM alpine:3.17.3
# RUN mkdir /app
# COPY --from=backend /app/backend /app/backend
# WORKDIR /app

# CMD ["./backend"]
# EXPOSE 3000
# ENV RUST_LOG="info"
# ENV PORT="3000"