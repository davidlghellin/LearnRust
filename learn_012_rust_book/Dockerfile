FROM rust:1.61 AS builder
COPY . .

RUN cargo build --release


FROM debian:buster-slim

COPY --from=builder ./target/release/learn_012_rust_book ./target/release/learn_012_rust_book

#CMD ["/target/release/learn_012_rust_book"]
ENTRYPOINT [ "./target/release/learn_012_rust_book" ]
