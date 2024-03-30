use std::io::{self, Write};

use super::face::{Face, Color};

#[derive(Clone, Debug)]
pub struct  RubikCube {
    pub faces: [Face; 6]
}

impl RubikCube {
    pub fn new() -> Self {
        let red_face = Face::new(Color::Red);
        let blue_face = Face::new(Color::Blue);
        let green_face = Face::new(Color::Green);
        let yellow_face = Face::new(Color::Yellow);
        let white_face = Face::new(Color::White);
        let orange_face = Face::new(Color::Orange);

        RubikCube {
            faces: [
                red_face,
                blue_face,
                green_face,
                yellow_face,
                white_face,
                orange_face
            ]
        }
    }

    pub fn select_face() -> usize {
        loop {
            println!("Select a face (0-5):");
            println!("0. Front");
            println!("1. Back");
            println!("2. Up");
            println!("3. Down");
            println!("4. Left");
            println!("5. Right");

            print!("Enter face number: ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            match input.trim().parse::<usize>() {
                Ok(face_index @ 0..=5) => return face_index,
                _ => println!("Invalid face number. Please enter a number between 0 and 5")
            }
        }    
    }

    pub fn read_from_input() -> Self {
        let mut cube = RubikCube::new();

        println!("Enter colors for each face:");
        println!("(R)ed, (B)lue, (G)reen, (O)range, (W)hite, (Y)ellow");
        println!("Enter the colors for each row of each face separated by spaces.");

        for _ in 0..6 {
            let face_idx = RubikCube::select_face();
            let face_name = match face_idx {
                0 => "Front",
                1 => "Back",
                2 => "Up",
                3 => "Down",
                4 => "Left",
                5 => "Right",
                _ => panic!("Invalid face index selected"),
            };

            println!("Enter colors for {} Face:", face_name);

            for i in 0..3 {
                print!("Row {}: ", i+1);
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");

                let colors: Vec<&str> = input.trim().split_whitespace().collect();
                if colors.len() != 3 {
                    println!("Please enter 3 colors for each row");
                    return  RubikCube::read_from_input();
                }

                for j in 0..3 {
                    let color_char = colors[j].chars().next().unwrap();
                    let color = match color_char {
                        'R' => Color::Red,
                        'B' => Color::Blue,
                        'G' => Color::Green,
                        'O' => Color::Orange,
                        'W' => Color::White,
                        'Y' => Color::Yellow,
                        _ => {
                            println!("Invalid color code.");
                            return  RubikCube::read_from_input();
                        }
                    };
                    cube.faces[face_idx].colors[i][j] = color;
                }
            }
        }

        cube
    }

    pub fn print(&self) {
        println!("<<<<<<<<<<<<Front Face>>>>>>>>>>>");
        self.faces[0].print_face();
        println!("<<<<<<<<<<<<Back Face>>>>>>>>>>>");
        self.faces[1].print_face();
        println!("<<<<<<<<<<<<Up Face>>>>>>>>>>>");
        self.faces[2].print_face();
        println!("<<<<<<<<<<<<Down Face>>>>>>>>>>>");
        self.faces[3].print_face();
        println!("<<<<<<<<<<<<Left Face>>>>>>>>>>>");
        self.faces[4].print_face();
        println!("<<<<<<<<<<<<Right Face>>>>>>>>>>>");
        self.faces[5].print_face();
    }
}