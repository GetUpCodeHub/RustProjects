use rand::Rng;

pub struct Enemy {
    pub x: usize,
    pub y: usize,
}

impl Enemy {
    pub fn new(width: usize) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            x: rng.gen_range(1..width - 1),
            y: 1,
        }
    }

    pub fn update(&mut self) {
        self.y += 1;
    }
}