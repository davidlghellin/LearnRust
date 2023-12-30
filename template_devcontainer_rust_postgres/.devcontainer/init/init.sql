CREATE DATABASE IF NOT exists `db_desde_sql`;
CREATE DATABASE IF NOT exists `db_desde_sql2`;

CREATE USER david @'localhost' identified by "secreto";
GRANT ALL ON `proyecto`.* to `david`@`%`;
