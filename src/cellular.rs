#[derive(Debug, Clone, Copy)]
pub struct Jitter<CellularNoise> {
    pub base: CellularNoise,
    pub jitter: f32,
}
