FROM rust:1.69 AS builder
COPY . .

RUN cargo build --release


FROM debian:buster-slim

COPY --from=builder ./target/release/learn_013_polars_dataframes ./target/release/learn_013_polars_dataframes

#CMD ["/target/release/learn_013_polars_dataframes"]
ENTRYPOINT [ "./target/release/learn_013_polars_dataframes" ]
