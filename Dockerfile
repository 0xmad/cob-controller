FROM rust:1.85-bullseye as builder

RUN apt-get update && \
    apt-get -y install ca-certificates cmake musl-tools libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest
WORKDIR /usr/src/app

RUN apk --no-cache add ca-certificates
COPY --from=builder /target/x86_64-unknown-linux-musl/release/rust-actix-web .

CMD ["cob-controller"]
