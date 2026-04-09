default:
  @just --list

pre-release:
  just check
  just doc
  just test
  cargo +stable clippy --all
  # https://github.com/crate-ci/typos
  typos
  # semver-checks@0.47.0 uses an older nightly with an incompatible nightly simd
  cargo +stable semver-checks --default-features

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
  cargo +1.82.0 check
  cargo +1.82.0 check --no-default-features --features libm
  cargo +1.82.0 check -p noise-functions-config
  cargo +1.82.0 check -p noise-functions-config --no-default-features --features libm

test:
  cargo +nightly test
  cargo +nightly test --features nightly-simd
  cargo +nightly test --no-default-features --features libm
  cargo +nightly test --no-default-features --features libm,nightly-simd

doc *args:
  cargo-insert-docs --allow-staged
  cargo +nightly rustdoc {{args}} --features nightly-simd -- --cfg docsrs