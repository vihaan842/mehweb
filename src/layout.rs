#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Position {
    // pixels
    Absolute(f32),
    // percent
    Relative(f32),
    // a combination of the two
    Combo(f32, f32),
}

// add two position enums together
impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
	match self {
	    Position::Absolute(pixels) => {
		match other {
		    Position::Absolute(other_pixels) => Position::Absolute(pixels+other_pixels),
		    Position::Relative(other_percent) => Position::Combo(pixels, other_percent),
		    Position::Combo(other_pixels, other_percent) => Position::Combo(pixels+other_pixels, other_percent),
		}
	    },
	    Position::Relative(percent) => {
		match other {
		    Position::Absolute(other_pixels) => Position::Combo(other_pixels, percent),
		    Position::Relative(other_percent) => Position::Relative(percent+other_percent),
		    Position::Combo(other_pixels, other_percent) => Position::Combo(other_pixels, percent+other_percent),
		}
	    },
	    Position::Combo(pixels, percent) => {
		match other {
		    Position::Absolute(other_pixels) => Position::Combo(pixels+other_pixels, percent),
		    Position::Relative(other_percent) => Position::Combo(pixels, percent+other_percent),
		    Position::Combo(other_pixels, other_percent) => Position::Combo(pixels+other_pixels, percent+other_percent),
		}
	    },
	}
    }
}

pub struct Rect {
    pub x: Position,
    pub y: Position,
    pub width: Position,
    pub height: Position,
    pub color: [f32;3],
}

impl Rect {
    pub fn new(x: Position, y: Position, width: Position, height: Position, color: [f32;3]) -> Rect {
	return Rect{x: x, y: y, width: width, height: height, color: color};
    }
}
