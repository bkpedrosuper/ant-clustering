
pub struct Config {
    pub dead_ants: usize,
    pub iterations: usize,
    pub ants: usize,
    pub radius: i32,
    pub border_size: f32,
    pub iter_per_mut: usize,
    pub finished: bool,
    pub base: String,
    
    // data metrics
    pub k1: f32,
    pub k2: f32,
    pub alpha: f32,
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
            base: "Square1-DataSet-400itens.txt".to_string(),

            k1: 0.3,
            k2: 0.3,
            alpha: 20.
        }
    }
}