set shell := ["nu", "-c"]

default:
  @just --list

pre-release:
  just check
  just doc
  just test
  cargo +stable clippy --all
  # cspell lint --gitignore "**" --exclude "from_open_simplex_2"
  cargo semver-checks

check:
  cargo fmt --all
  cargo clippy --all --tests
  cargo clippy --all --tests --features nightly-simd
  cargo clippy --all --tests --features nightly-simd,libm
  cargo clippy --all --tests --no-default-features --features libm
  cargo clippy --all --tests --no-default-features --features libm,nightly-simd
  cargo clippy --all --no-default-features --features libm -p noise-functions-config
  just check-msrv

check-msrv:
  cargo ('+' + (open Cargo.toml).package.rust-version) check
  cargo ('+' + (open Cargo.toml).package.rust-version) check --no-default-features --features libm
  cargo ('+' + (open Cargo.toml).package.rust-version) check -p noise-functions-config
  cargo ('+' + (open Cargo.toml).package.rust-version) check -p noise-functions-config --no-default-features --features libm

test:
  cargo test
  cargo test --features nightly-simd
  cargo test --no-default-features --features libm
  cargo test --no-default-features --features libm,nightly-simd

doc *args:
  cargo rustdoc {{args}} --features nightly-simd,document-features -- --cfg docsrs

inspect-asm:
  just crates/inspect-asm/inspect-asm