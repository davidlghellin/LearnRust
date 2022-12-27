FROM rust:1.61 AS builder
COPY . .

RUN cargo build --release


FROM debian:buster-slim

COPY --from=builder ./target/release/learn_007_actix_web_001 ./target/release/learn_007_actix_web_001

#CMD ["/target/release/learn_007_actix_web_001"]
ENTRYPOINT [ "/target/release/learn_007_actix_web_001" ]
