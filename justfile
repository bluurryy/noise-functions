set shell := ["nu", "-c"]

default:
  cargo fmt --all
  cargo clippy --tests --all
  cargo clippy --no-default-features --features libm
  cargo clippy --no-default-features --features libm -p noise-functions-config

all: default
  just doc
  just test
  cargo +stable clippy --all
  cargo ('+' + (open Cargo.toml).package.rust-version) check
  cargo ('+' + (open Cargo.toml).package.rust-version) check -p noise-functions-config
  cspell lint --gitignore "**" --exclude "*.asm"
  
publish:
  cargo semver-checks
  cargo publish

test:
  cargo test --features nightly-simd

doc *args:
  cargo rustdoc {{args}} --features nightly-simd,document-features -- --cfg docsrs

inspect-asm:
  just crates/inspect-asm/inspect-asm