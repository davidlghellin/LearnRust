FROM rust:1.61 AS builder
COPY . .

RUN cargo build --release


FROM debian:buster-slim

COPY --from=builder ./target/release/learn_008_actix_web_002 ./target/release/learn_008_actix_web_002

#CMD ["/target/release/learn_007_actix_web_001"]
ENTRYPOINT [ "/target/release/learn_008_actix_web_002" ]
