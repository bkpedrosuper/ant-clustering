
pub struct Config {
    pub dead_ants: usize,
    pub max_iter: usize,
    pub ants: usize,
    pub radius: usize,
}

impl Config {
    pub fn new(
        dead_ants: usize,
        max_iter: usize,
        ants: usize,
        radius: usize,
    ) -> Self {

        Self {
            dead_ants,
            max_iter,
            ants,
            radius,
        }
    }
}