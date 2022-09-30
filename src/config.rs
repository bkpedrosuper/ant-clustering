
pub struct Config {
    pub dead_ants: usize,
    pub iterations: usize,
    pub ants: usize,
    pub radius: i32,
    pub border_size: f32,
    pub iter_per_mut: usize,
    pub finished: bool,
}

impl Config {
    pub fn new(
        dead_ants: usize,
        iterations: usize,
        ants: usize,
        radius: i32,
        border_size: f32,
        iter_per_mut: usize,
    ) -> Self {

        Self {
            dead_ants,
            iterations,
            ants,
            radius,
            border_size,
            iter_per_mut,
            finished: false,
        }
    }
}