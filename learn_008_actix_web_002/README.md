# Actix 002

En esta ocasión vamos a seguir aprendiendo con Rust de la mano de la siguiente lista de reproducción https://www.youtube.com/watch?v=m0LuiS9squU&list=PLojDVPvSO1Di_QEnvDOI5FIeieqYpm1xd 

```sh
http get 127.0.0.1:8080
curl -i  127.0.0.1:8080
```

Generamos el docker y ahora vamos a comprobar que se pasa datos buenos
```sh
docker build -t actix-web-docker-api .
docker run --network host -p 8080:8080 -e PORT=8080 -e HOST="127.0.0.1" actix-web-docker-api
```

```sh
curl -i  127.0.0.1:8080/user/af08097e-e5b2-4ee6-a5b3-d6a0aaa72140
http get 127.0.0.1:8080/user/af08097e-e5b2-4ee6-a5b3-d6a0aaa72140
# not found
curl -i  127.0.0.1:8080/user/af08097e-e5b2-4ee6-a5b3-d6a0aaa72141
http get 127.0.0.1:8080/user/af08097e-e5b2-4ee6-a5b3-d6a0aaa72141
# invalid
curl -i  127.0.0.1:8080/user/af08097e-e5b2-4ee6-a5b3-d6a0aaa721
http get 127.0.0.1:8080/user/af08097e-e5b2-4ee6-a5b3-d6a0aaa721
```

hemos añadido el `cargo-watch` para actualizar las compilaciones con
```
cargo watch -x run
```
