#![cfg(any())]

mod regression;

#[cfg(feature = "nightly-simd")]
mod simd_regression;

#[cfg(feature = "nightly-simd")]
mod usage;

#[cfg(miri)]
const DEFAULT_N: usize = 3;

#[cfg(not(miri))]
const DEFAULT_N: usize = 10;

fn test_n() -> usize {
    std::env::var("TEST_N").ok().map(|s| s.parse().unwrap()).unwrap_or(DEFAULT_N)
}

fn test_seed() -> u64 {
    std::env::var("TEST_SEED").ok().map(|s| s.parse().unwrap()).unwrap_or(0)
}
