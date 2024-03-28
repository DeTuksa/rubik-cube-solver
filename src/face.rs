#[derive(Debug, Clone, Copy)]
pub enum  Color {
    Red,
    Blue,
    Green,
    Orange,
    White,
    Yellow
}

#[derive(Debug)]
pub struct  Face {
    pub colors: [[Color; 3]; 3]
}

impl Face {
    pub fn new(color: Color) -> Self {
        Face { colors: [[color; 3]; 3] }
    }

    pub fn rotate_clockwise(&mut self) {
        self.colors = [
            [self.colors[2][0], self.colors[1][0], self.colors[0][0]],
            [self.colors[2][1], self.colors[1][1], self.colors[0][1]],
            [self.colors[2][2], self.colors[1][2], self.colors[0][2]],
        ]
    }

    pub fn rotate_anti_clockwise(&mut self) {
        self.colors = [
            [self.colors[0][2], self.colors[1][2], self.colors[2][2]],
            [self.colors[0][1], self.colors[1][1], self.colors[2][1]],
            [self.colors[0][0], self.colors[1][0], self.colors[2][0]],
        ]
    }
}