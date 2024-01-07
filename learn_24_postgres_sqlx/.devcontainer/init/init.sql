-- Sino carga reviar permisos, parece que no carga
create database mydb;
create user david with CREATEDB encrypted password 'david';
grant all privileges on database mydb to david;

CREATE TABLE postgres.public.user(nombre VARCHAR(32), apellido VARCHAR(32), id VARCHAR(64));
INSERT INTO  postgres.public.user(nombre, apellido, id) VALUES('David', 'Lopez', '1');
