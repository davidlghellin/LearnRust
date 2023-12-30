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
