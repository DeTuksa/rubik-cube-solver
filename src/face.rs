// use std::io::{self, Write};

#[derive(Debug, Clone, Copy)]
pub enum  Color {
    Red,
    Blue,
    Green,
    Orange,
    White,
    Yellow
}

impl Color {
    pub fn as_char(&self) -> char {
        match *self {
            Color::Red => 'R',
            Color::Blue => 'B',
            Color::Green => 'G',
            Color::Orange => 'O',
            Color::White => 'W',
            Color::Yellow => 'Y'
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

    pub fn print_face(&self) {
        for row in &self.colors {
            for color in row {
                let color_code = match color {
                    Color::Red => "\x1b[31m",   // Red
                    Color::Blue => "\x1b[34m",  // Blue
                    Color::Green => "\x1b[32m", // Green
                    Color::Orange => "\x1b[38;5;214m",// Orange
                    Color::White => "\x1b[37m", // White
                    Color::Yellow => "\x1b[33m",// Yellow
                };
                print!("{}{} \x1b[0m", color_code, color.as_char());
                // io::stdout().flush();
            }
            println!();
        }
    }
}