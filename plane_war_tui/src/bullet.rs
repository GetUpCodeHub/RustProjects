pub struct Bullet {
    pub x: usize,
    pub y: usize,
}

impl Bullet {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn update(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }
}