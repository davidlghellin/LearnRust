# Amux

```sh
cargo install cargo-watch
cargo watch -x run
```
https://www.youtube.com/watch?v=XZtlD_m59sM

```sh
http get 127.0.0.1:8080/hello
http get 127.0.0.1:8080/hello name=david

curl  -i 'http://localhost:8080/hello?name=david'
http get "127.0.0.1:8080/hello?name=david"
```


cargo watch -q -c -w src/ -x run