docker compose up -d sail-server \
 docker compose up pyspark-client

docker logs -f sail-pyspark-sail-server-1

docker compose up pyspark-client

---

cd
sail

docker build --build-arg PYSAIL_VERSION=0.2.5 -t sail .

cd ..
docker compose up --abort-on-container-exit
docker compose up --build
