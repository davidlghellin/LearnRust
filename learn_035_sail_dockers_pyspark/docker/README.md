docker network create sail-net

docker run --rm -d --name sail-server -v ./data:/stage --network sail-net -p 50051:50051 sail spark server --ip 0.0.0.0 --port 50051

docker logs -f sail-server

docker run --rm -it --network sail-net pyspark-client

docker run --rm -it \
 --name pyspark-client \
 --network sail-net \
 -v "$PWD/client:/app" \
 pyspark-client

docker run --rm -it \
 --name pyspark-client \
 --network sail-net \
 -v "$PWD/client:/app" \
 pyspark-client python app.py

## Makefile

sin probar

```make
# Variables
IMAGE_NAME = pyspark-client
SCRIPT ?= client.py
CLIENT_DIR = ./client
NETWORK = sail-net

# Construir imagen del cliente
build:
	docker build -t $(IMAGE_NAME) $(CLIENT_DIR)

# Lanzar servidor Sail (modo detached)
run-server:
	docker run -d --rm \
		--name sail-server \
		--network $(NETWORK) \
		-p 50051:50051 \
		-e RUST_LOG=info \
		sail spark server --ip 0.0.0.0 --port 50051

# Ejecutar cliente con script (por defecto: client.py)
run-client:
	docker run --rm -it \
		--name pyspark-client \
		--network $(NETWORK) \
		-v $(PWD)/client:/app \
		-w /app \
		$(IMAGE_NAME) python $(SCRIPT)

# Ver logs del servidor
logs-server:
	docker logs -f sail-server

# Parar servidor Sail
stop-server:
	docker stop sail-server

# Atajo para todo junto: construir, lanzar servidor, y correr el cliente
run: build run-server run-client

# Atajo con script personalizado
run-script:
	$(MAKE) build
	$(MAKE) run-server
	docker run --rm -it \
		--name pyspark-client \
		--network $(NETWORK) \
		-v $(PWD)/client:/app \
		-w /app \
		$(IMAGE_NAME) python $(SCRIPT)

.PHONY: build run run-server run-client stop-server logs-server run-script

```

make run # build + start server + run client.py
make SCRIPT=app.py run-script # run otro script
make logs-server # ver logs del servidor Sail
make stop-server # parar el contenedor sail-server
