# Pokedex

Siguiendo el https://alexis-lozano.com/hexagonal-architecture-in-rust-1/ se intenta aprender y crear los módulos independientes para la arquitectura hexagonal

touch Cargo.toml
```toml
[workspace]
members = []

[workspace. Dependencies]
# We will add all our common dependent crates for our sub crates to share
# the same version. How cool is that :D
 ```
 ```sh
cargo new --vcs=none --lib domain

mkdir -p domain/src/models
mkdir -p domain/src/ports/input
mkdir -p domain/src/ports/output
touch domain/src/mod.rs
touch domain/src/models/mod.rs
touch domain/src/ports/mod.rs
touch domain/src/ports/input/mod.rs
touch domain/src/ports/output/mod.rs

cargo new --vcs=none pokedex
 ```