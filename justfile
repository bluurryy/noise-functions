set shell := ["nu", "-c"]

default:
  @just --list

pre-release:
  just check
  just doc
  just test
  cargo +stable clippy --all
  # https://github.com/crate-ci/typos
  typos
  cargo +stable semver-checks

check:
  cargo +nightly fmt --all
  cargo +nightly clippy --all --tests
  cargo +nightly clippy --all --tests --features nightly-simd
  cargo +nightly clippy --all --tests --features nightly-simd,libm
  cargo +nightly clippy --all --tests --no-default-features --features libm
  cargo +nightly clippy --all --tests --no-default-features --features libm,nightly-simd
  cargo +nightly clippy --all --no-default-features --features libm -p noise-functions-config
  just check-msrv

check-msrv:
  cargo ('+' + (open Cargo.toml).package.rust-version) check
  cargo ('+' + (open Cargo.toml).package.rust-version) check --no-default-features --features libm
  cargo ('+' + (open Cargo.toml).package.rust-version) check -p noise-functions-config
  cargo ('+' + (open Cargo.toml).package.rust-version) check -p noise-functions-config --no-default-features --features libm

test:
  cargo +nightly test
  cargo +nightly test --features nightly-simd
  cargo +nightly test --no-default-features --features libm
  cargo +nightly test --no-default-features --features libm,nightly-simd

doc *args:
  cargo +nightly rustdoc {{args}} --features nightly-simd,document-features -- --cfg docsrs