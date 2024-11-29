set shell := ["nu", "-c"]

default:
  @just --list

pre-release:
  just check
  just doc
  just test
  cargo +stable clippy --all
  cargo ('+' + (open Cargo.toml).package.rust-version) check
  cargo ('+' + (open Cargo.toml).package.rust-version) check -p noise-functions-config
  cspell lint --gitignore "**" --exclude "*.asm"
  cargo semver-checks

check:
  cargo fmt --all
  cargo clippy --tests --all
  cargo clippy --no-default-features --features libm
  cargo clippy --no-default-features --features libm -p noise-functions-config
  
test:
  cargo test
  cargo test --features libm
  cargo test --features nightly-simd
  cargo test --features nightly-simd,libm

doc *args:
  cargo rustdoc {{args}} --features nightly-simd,document-features -- --cfg docsrs

inspect-asm:
  just crates/inspect-asm/inspect-asm