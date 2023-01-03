# 

```
mycli mysql://gestor@localhost:3306/proyecto -p secreto
mycli mysql://gestor@db:3306/proyecto -p secreto
```

```sh
cargo init
cargo install diesel_cli
diesel setup
```

```sh
diesel migration generate create_plants
diesel migration run
```
continuar con https://medium.com/@rameshovyas/a-step-by-step-guide-to-access-mysql-database-in-rust-using-diesel-orm-41836062bb6d
