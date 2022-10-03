#[derive(Default, Clone, Copy, Debug)]
pub struct Food {
    pub d1: f32,
    pub d2: f32,
    pub label: usize,
}

impl Food {
    pub fn new(d1: f32, d2: f32, label: usize) -> Self {
        Self {
            d1, d2, label
        }
        
    }

    pub fn dist(el1: self::Food, el2: self::Food) -> f32 {
        // Euclidian Distance
        ((el1.d1 - el2.d1).powi(2) + (el1.d2 - el2.d2).powi(2)).sqrt()
    }

}