# Rust Tutorials

## [Install Rust][install-rust]
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Configure Shell
```shell
source "$HOME/.cargo/env"
```

## Build
```shell
cargo build
```

## Run

### Basics

#### Binaries
```shell
cargo run --bin basics <Name>
```

#### Data Structures
```shell
cargo run --bin ds
```

#### FileIO
```shell
cargo run --bin fileio <Filename>
```

#### Format Specifiers
```shell
cargo run --bin format
```

### HTTP Requests

#### GET
```shell
cargo run --bin get <URL>
```

## Uninstall Rust
```shell
rustup self uninstall
```

[install-rust]: https://www.rust-lang.org/tools/install
