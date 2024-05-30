# TODO app

http://localhost:3030/swagegr-ui/

get
```sh
http GET http://localhost:3030/todos
```

insertar
```sh
echo '{"title": "el titulo del todo mypass"}' | http POST http://localhost:3030/todos
```

update
```sh
echo '{"id": 3, "title": "el titulo actualizado","completed":true}' | http PATCH http://localhost:3030/todos/3
```

borrar
```sh
echo '{"id": "4"}' | http DELETE http://localhost:3030/todos/4
```
