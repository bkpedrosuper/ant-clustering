
pub struct Config {
    pub dead_ants: usize,
    pub max_iter: usize,
    pub ants: usize,
    pub radius: i32,
    pub border_size: f32,
}

impl Config {
    pub fn new(
        dead_ants: usize,
        max_iter: usize,
        ants: usize,
        radius: i32,
        border_size: f32,
    ) -> Self {

        Self {
            dead_ants,
            max_iter,
            ants,
            radius,
            border_size,
        }
    }
}