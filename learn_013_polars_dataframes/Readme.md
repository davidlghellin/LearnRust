# Book

```sh
cargo install cargo-watch
cargo watch -x run
```

De momento tenomos que lanzar así el docker
```sh
docker build -t polar-rust .
docker run polar-rust:latest
```

```sh
docker build -t myip:alpine -f Dockerfile.alpine .
docker run -ti --rm myip:alpine
```