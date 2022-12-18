use rust_embed::RustEmbed;
use salvo::serve_static::static_embed;
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

//mi carpeta de fronted se encuenta
#[derive(RustEmbed)]
#[folder = "../learn_001_salvo_tutorial_client/client/dist/"]
struct Assets;

#[tokio::main]
async fn main() {
    let router = Router::new()
        // Nos puede servir para crear API
        .push(Router::new().path("").get(hello_world))
        .push(Router::new().path("about").get(about))
        .push(Router::new().path("service").get(service))
        .push(
            Router::with_path("<**path>").get(static_embed::<Assets>().with_fallback("index.html")),
        );
    // y esto para servir el contenido estático
    //     .push(
    //         Router::with_path("<**path>").get(
    //             StaticDir::new(["public"])
    //                 .with_defaults("index2.html")
    //                 .with_listing(true),
    //         ),
    //     );

    let __router2 =
        Router::with_path("<**path>").get(static_embed::<Assets>().with_fallback("index.html"));

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .serve(router)
        .await;
}
