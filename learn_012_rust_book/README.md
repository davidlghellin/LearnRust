# Book

```sh
cargo install cargo-watch
cargo watch -x run
```

De momento tenomos que lanzar así el docker
```sh
docker build -t hola-web-rust .
docker run --network host -p 8000:8000 hola-web-rust:latest
```
