```
cd actor_server
docker build --no-cache -t actor_server:latest .
docker run -d --name actor_server -p 7878:7878 actor_server:latest
```

```
cd actor_client
docker build --no-cache -t actor_client:latest .
docker run --rm actor_client:latest
docker run --rm -e SERVER_IP=host.docker.internal actor_client:latest
```
