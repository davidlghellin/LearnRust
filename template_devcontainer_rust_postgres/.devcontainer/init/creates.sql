create table if not exists proyecto.tiendas(
    id int auto_increment primary key,
    nombre varchar(100) not null,
    tlf varchar(13) null
);

INSERT INTO proyecto.tiendas (nombre,tlf) VALUES ("Tienda1","1234");
INSERT INTO proyecto.tiendas (nombre,tlf) VALUES ("Tienda2","5678");
