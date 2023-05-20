FROM rust:1.69 AS builder
COPY . .

RUN cargo build --release


FROM debian:buster-slim

COPY --from=builder ./target/release/learn_016_actix_web_003 ./target/release/learn_016_actix_web_003

CMD ["/target/release/learn_016_actix_web_003"]
