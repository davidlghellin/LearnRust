# Actix

https://www.youtube.com/playlist?list=PLECOtlti4Psr4hXVX5GuSvLKp0-RZjz93

```sh
cargo install cargo-watch
cargo watch -x run
```

```shell
export DOCKER_BUILDKIT=1
docker build -t  todo-app .

docker run --network host -p 8080:8080 -it todo-app
```

```shell
http get 127.0.0.1:8080 
curl -i  http://127.0.0.1:8080
```
