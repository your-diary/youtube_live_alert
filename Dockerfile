FROM alpine:latest

WORKDIR /youtube_live_alert

COPY Cargo.toml Cargo.lock ./
COPY src/ src/

ENV RUSTFLAGS='--allow unused_parens'

RUN apk add rust cargo pkgconfig libressl-dev
RUN cargo build --release

CMD ["./target/release/youtube_live_alert"]

