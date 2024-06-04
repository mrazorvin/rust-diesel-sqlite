# Rust + Diesel + Sqlite example

to run example use `cargo run um` or `cargo build --release` to build production executable

## Used Tools & Commands 
* Diesel CLI
    * `diesel migration generate create_posts`
    * `diesel migration generate fixture_posts`
    * `diesel print-schema --database-url "file:db.sqlite" > src/schema.rs`
* DBeaver 