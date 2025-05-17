# Etapa de build
FROM rust:1.77 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Etapa final con dependencias básicas (glibc)
FROM debian:bookworm-slim

# Instala solo lo necesario para ejecutar binarios
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/actor_server /app/server

ENTRYPOINT ["/app/server"]
