Instalamos el samply

`
cargo install --locked samply
echo '1' | sudo tee /proc/sys/kernel/perf_event_paranoid
cargo build && samply record ./target/debug/profilin_example
`

`
cargo run --features dhat-heap --bin profiling_dhat
`

visor
https://nnethercote.github.io/dh_view/dh_view.html
