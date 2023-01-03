create database if not exists prueba;

create table if not exists proyecto.usuarios_pass(
    id int auto_increment primary key,
    usuario varchar(20) not null,
    password varchar(64) not null
);

INSERT INTO proyecto.usuarios_pass (usuario,password) VALUES ("david","1234");
INSERT INTO proyecto.usuarios_pass (usuario,password) VALUES ("1234",sha2('1234',256));
INSERT INTO proyecto.usuarios_pass (usuario,password) VALUES ("gestor",sha2('pass',256));
