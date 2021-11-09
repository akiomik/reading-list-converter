reading-list-converter
======================

[![Rust CI](https://github.com/akiomik/reading-list-converter/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/akiomik/reading-list-converter/actions/workflows/rust-ci.yml)

`rlconv` is a cli tool to convert an exported reading list into other format that is importable.

## Usage

```sh
rlconv --input pocket.html --input-format Pocket \
       --output safari.html --output-format Safari
```

## Build

```sh
cargo build --release
```
