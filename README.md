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
cargo run --bin basics Vignesh
```

#### Data Structures
```shell
cargo run --bin ds
```

#### Format Specifiers
```shell
cargo run --bin format
```

### FileIO

#### Read File
```shell
cargo run --bin read-file README.md
```

#### Write File
```shell
cargo run --bin write-file hello.txt
```

### HTTP Requests

#### GET
```shell
cargo run --bin get https://jarvis.vigneshrao.com/health
```

## Uninstall Rust
```shell
rustup self uninstall
```

[install-rust]: https://www.rust-lang.org/tools/install
