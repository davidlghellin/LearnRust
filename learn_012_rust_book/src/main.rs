//! main.rs

use learn_012_rust_book::run;
#[tokio::main]
async fn main() -> std::io::Result<()> {
// Bubble up the io::Error if we failed to bind the address
// Otherwise call .await on our Server
run()?.await
}