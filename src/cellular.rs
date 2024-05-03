#[derive(Debug, Clone, Copy)]
pub struct Jitter<CellularNoise> {
    pub noise: CellularNoise,
    pub jitter: f32,
}
