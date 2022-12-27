# Actix

Creditos a https://www.youtube.com/playlist?list=PLTeVz4vFc2T40rXK79TLzM5-bghdK41EC 

```
sudo apt install net-tools
netstat -ntpl
netstat -putona | grep 8080
```

```
http get 127.0.0.1:8080/david
curl http://localhost:8080/david

for i in $(seq 1 100); do curl http://localhost:8080/david ; done
```

Post
```sh
curl -X POST http://localhost:8080/api/playlist/ -H 'Content-Type: application/json' -d '{"name": "Diciembre 2020", "song": "trance"}'

http post 127.0.0.1:8080/api/playlist/  name="Diciembre 2020" song="trance"
```

## Docker
Para crear el docker haremos:
```sh
docker build -t actix-web-docker .
```
Para ejecutarlo lanzaremos el siguiente comando
```sh
docker run --network host -p 8080:8080 -e PORT=8080 -e HOST="127.0.0.1" actix-web-docker-example
```
Hemos tenido que añadir el `--network host` para poder hacer los curls.
Además como hemos creado lo de las variables de entorno debemos pasarlas.
