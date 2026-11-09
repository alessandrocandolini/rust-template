# Rust starter template

Opinionated `cargo-generate` template with PBT, GHA, etc

Assuming the `cargo-generate` engine is installed in the system (e.g., via nix), a new project folder can be generated using
```sh
cargo generate alessandrocandolini/rust-template --name <name>
```

or if the template is available locally under `~/rust-template`,
```
cargo generate --path ~/rust-template --name <name>
```

In the generated template you can run
```
cargo run
cargo test
cargo fmt 
cargo clippy --all-targets -- -D warnings
cargo build --release
```
