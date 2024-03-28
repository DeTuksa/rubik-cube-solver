use super::face::{Face, Color};

#[derive(Clone)]
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
}