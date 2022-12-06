use salvo::prelude::*;

#[handler]
async fn hello_world() -> &'static str {
    "hola mundo"
}
#[tokio::main]
async fn main() {
    let router = Router::new().get(hello_world);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .serve(router)
        .await;
}
