// https://github.com/nnethercote/dhat-rs/blob/master/examples/heap.rs
// This is a very simple example of how to do heap profiling of a program. You
// may want to create a feature called `dhat-heap` in your `Cargo.toml` and
// uncomment the `#[cfg(feature = "dhat-heap")]` attributes.

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let text="Hello, world!".to_string();
    println!("{}", text);
}

fn sumar(a: i32, b: i32) -> i32 {
    a + b
}
