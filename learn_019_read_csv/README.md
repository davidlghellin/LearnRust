# CSV-WORK


```
cargo new csv-work
```

Para poder usar el docker podemos hacer uso de los volumenes.
```shell
#docker build -t  my-read-csv -f DockerfileScratch .
docker build --build-arg UID=$(id -u) --build-arg USER=csv-work -t my-read-csv -f DockerfileScratch . 

docker run -v ~/Documentos/LearnRust/learn_019_read_csv/input/:/input my-read-csv
```

Añadimos los log en ficheros (la carpeta destino tiene que existir y tener permisos (bruscamente 777))
```shell
docker run -v ~/Documentos/LearnRust/learn_019_read_csv/input/:/input -v ~/Documentos/LearnRust/learn_019_read_csv/log/:/log my-read-csv
docker run -v ~/Documentos/LearnRust/learn_019_read_csv/input/:/input -v /tmp/:/log my-read-csv
```
