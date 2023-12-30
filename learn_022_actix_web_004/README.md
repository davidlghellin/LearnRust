# Pizzas

https://www.youtube.com/watch?v=Rnw-x21kGaA

curl -i  http://127.0.0.1:8080/pizzas

```shell
docker build -t  my-rust-pizzas -f Dockerfile .  
docker run --network host -p 8080:8080 -e PORT=8080 -e HOST="127.0.0.1" -it my-rust-pizzas
```

```shell
cargo install cargo-watch
cargo watch -x run
```

```shell
http get   127.0.0.1:8080/pizzas
http post  127.0.0.1:8080/comprarpizza pizza_name=
http post  127.0.0.1:8080/comprarpizza pizza_name="la ultima"
http patch 127.0.0.1:8080/actualizarpizza/12345
```

```shell
docker run --rm --pull always -p 8000:8000 surrealdb/surrealdb:latest start --log trace --auth --user root --pass root
http get 127.0.0.1:8000/version
```

Creamos la red para conectar los dockers
```shell
docker network create myNetwork
docker ps
docker network connect myNetwork 0169c632ceb9
docker network connect myNetwork 1822ab03621f
```

Instalamos en el devcontainer el ping para ver si creamos bien la red

```shell
apt
apt update
apt-get install -y iputils-ping
```