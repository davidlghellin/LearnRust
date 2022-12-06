use salvo::{prelude::*, serve_static::StaticDir};

#[handler]
async fn hello_world() -> &'static str {
    "hola mundo"
}

#[handler]
async fn about() -> &'static str {
    "about"
}
#[handler]
async fn service() -> &'static str {
    "service"
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        // Nos puede servir para crear API
        .push(Router::new().path("").get(hello_world))
        .push(Router::new().path("about").get(about))
        .push(Router::new().path("service").get(service))
        // y esto para servir el contenido estático
        .push(
            Router::with_path("<**path>").get(
                StaticDir::new(["public"])
                    .with_defaults("index.html")
                    .with_listing(true),
            ),
        );

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .serve(router)
        .await;
}
