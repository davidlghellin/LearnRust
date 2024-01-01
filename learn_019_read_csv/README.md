# CSV-WORK


```
cargo new csv-work
```

Para poder usar el docker podemos hacer uso de los volumenes.
```shell
docker build -t  my-read-csv -f DockerfileScratch .
docker run -v ~/Documentos/LearnRust/learn_019_read_csv/input/:/input my-read-csv
```
