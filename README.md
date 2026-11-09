[![CI](https://github.com/alessandrocandolini/rust-template/actions/workflows/ci.yml/badge.svg)](https://github.com/alessandrocandolini/rust-template/actions/workflows/ci.yml)

# Rust starter template

Opinionated `cargo-generate` template with PBT, GHA, etc

Assuming the `cargo-generate` engine is installed in the system (e.g., via nix), a new project folder can be generated using
```sh
cargo generate alessandrocandolini/rust-template --name <name>
```
or using SSH keys
```
cargo generate --git ssh://git@github.com/alessandrocandolini/rust-template.git --name <name>
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
## Upload test coverage to codecov

⚠️Tokenless upload of code coverage reports to [Codecov](https://app.codecov.io/) is no longer supported by [codecov-action v7](https://github.com/codecov/codecov-action).

In order for the GitHub Actions CI of the generated project to be able to automatically upload the generated test coverage reports to codecov, manually set the `CODECOV_TOKEN` repository secret in GitHub. 

The token can be found in Codecov under `Settings` > `Global Upload Token`.
The secret can be set in github repo under `Settings > Secrets and variables > Actions > new secret`. 
