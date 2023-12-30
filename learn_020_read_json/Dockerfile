FROM rust:1.61 AS builder
COPY ./json-work .

RUN cargo build --release


FROM debian:buster-slim

COPY --from=builder ./target/release/json-work ./target/release/json-work

CMD ["/target/release/json-work"]
