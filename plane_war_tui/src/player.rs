pub struct Player {
    pub x: usize,
    pub y: usize,
}

impl Player {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn move_left(&mut self) {
        if self.x > 1 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self, width: usize) {
        if self.x < width - 2 {
            self.x += 1;
        }
    }
}