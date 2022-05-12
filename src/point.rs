#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

impl Point {
    #[must_use]
    pub const fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    #[cfg(feature = "rand")]
    #[must_use]
    pub fn random(width: u8, height: u8) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..width - 1);
        let y = rng.gen_range(0..height - 1);
        Self { x, y }
    }
}
