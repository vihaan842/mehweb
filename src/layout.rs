use adw::gtk::cairo::{FontSlant, FontWeight};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Distance {
    // pixels
    Absolute(f64),
    // percent
    Relative(f64),
    // a combination of the two
    Combo(f64, f64),
}

// add two position enums together
impl std::ops::Add for Distance {
    type Output = Self;

    fn add(self, other: Self) -> Self {
	match self {
	    Distance::Absolute(pixels) => {
		match other {
		    Distance::Absolute(other_pixels) => Distance::Absolute(pixels+other_pixels),
		    Distance::Relative(other_percent) => Distance::Combo(pixels, other_percent),
		    Distance::Combo(other_pixels, other_percent) => Distance::Combo(pixels+other_pixels, other_percent),
		}
	    },
	    Distance::Relative(percent) => {
		match other {
		    Distance::Absolute(other_pixels) => Distance::Combo(other_pixels, percent),
		    Distance::Relative(other_percent) => Distance::Relative(percent+other_percent),
		    Distance::Combo(other_pixels, other_percent) => Distance::Combo(other_pixels, percent+other_percent),
		}
	    },
	    Distance::Combo(pixels, percent) => {
		match other {
		    Distance::Absolute(other_pixels) => Distance::Combo(pixels+other_pixels, percent),
		    Distance::Relative(other_percent) => Distance::Combo(pixels, percent+other_percent),
		    Distance::Combo(other_pixels, other_percent) => Distance::Combo(pixels+other_pixels, percent+other_percent),
		}
	    },
	}
    }
}
impl std::ops::Sub for Distance {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
	match self {
	    Distance::Absolute(pixels) => {
		match other {
		    Distance::Absolute(other_pixels) => Distance::Absolute(pixels-other_pixels),
		    Distance::Relative(other_percent) => Distance::Combo(pixels, -other_percent),
		    Distance::Combo(other_pixels, other_percent) => Distance::Combo(pixels-other_pixels, -other_percent),
		}
	    },
	    Distance::Relative(percent) => {
		match other {
		    Distance::Absolute(other_pixels) => Distance::Combo(-other_pixels, percent),
		    Distance::Relative(other_percent) => Distance::Relative(percent-other_percent),
		    Distance::Combo(other_pixels, other_percent) => Distance::Combo(-other_pixels, percent-other_percent),
		}
	    },
	    Distance::Combo(pixels, percent) => {
		match other {
		    Distance::Absolute(other_pixels) => Distance::Combo(pixels-other_pixels, percent),
		    Distance::Relative(other_percent) => Distance::Combo(pixels, percent-other_percent),
		    Distance::Combo(other_pixels, other_percent) => Distance::Combo(pixels-other_pixels, percent-other_percent),
		}
	    },
	}
    }
}
impl std::ops::AddAssign for Distance {
    fn add_assign(&mut self, other: Self) {
	*self = *self+other;
    }
}
impl std::ops::SubAssign for Distance {
    fn sub_assign(&mut self, other: Self) {
	*self = *self-other;
    }
}
impl std::cmp::PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
	match self {
	    Distance::Absolute(pixels) => {
		match other {
		    Distance::Absolute(other_pixels) => pixels.partial_cmp(other_pixels),
		    Distance::Relative(_) => None,
		    Distance::Combo(_, _) => None,
		}
	    },
	    Distance::Relative(percent) => {
		match other {
		    Distance::Absolute(_) => None,
		    Distance::Relative(other_percent) => percent.partial_cmp(other_percent),
		    Distance::Combo(_, _) => None,
		}
	    },
	    Distance::Combo(pixels, percent) => {
		match other{
		    Distance::Absolute(_) => None,
		    Distance::Relative(_) => None,
		    Distance::Combo(other_pixels, other_percent) => {
			if pixels.partial_cmp(other_pixels) == percent.partial_cmp(other_percent) {
			    pixels.partial_cmp(other_pixels)
			} else {
			    None
			}
		    }
		}
	    },
	}
    }
}

impl From<String> for Distance {
    fn from(s: String) -> Distance {
	if s.ends_with("em") {
	    Distance::Absolute(s.trim_end_matches("em").parse::<f64>().unwrap()*crate::rules::DEFAULT_FONT_SIZE as f64)
	} else if s.ends_with("%") {
	    Distance::Relative(s.trim_end_matches('%').parse::<f64>().unwrap()/100.)
	} else {
	    Distance::Absolute(s.trim_end_matches("px").parse::<f64>().unwrap())
	}
    }
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct Rect {
//     pub x: Distance,
//     pub y: Distance,
//     pub width: Distance,
//     pub height: Distance,
//     pub visual_height: Distance,
//     pub color: [f64;4],
//     pub label: Option<Label>,
// }

// impl Rect {
//     pub fn new(x: Distance, y: Distance, width: Distance, height: Distance, visual_height: Distance, color: [f64;4]) -> Rect {
// 	return Rect{x: x, y: y, width: width, height: height, visual_height: visual_height, color: color, label: None};
//     }
//     pub fn new_with_label(x: Distance, y: Distance, color: [f64;4], font_size: Distance, text: String, weight: FontWeight, slant: FontSlant) -> Rect {
// 	let label = Label{text: text, weight: weight, slant: slant};
// 	return Rect{x: x, y: y, width: Distance::Relative(1.), height: font_size, visual_height: font_size, color: color, label: Some(label)};
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutBox {
    pub margin_left: Distance,
    pub margin_top: Distance,
    pub margin_right: Distance,
    pub margin_bottom: Distance,
    pub padding_left: Distance,
    pub padding_top: Distance,
    pub padding_right: Distance,
    pub padding_bottom: Distance,
    pub height: Option<Distance>,
    pub width: Option<Distance>,
    pub visual_width: Distance,
    pub visual_height: Option<Distance>,
    pub content: Content,
}

impl LayoutBox {
    pub fn empty() -> LayoutBox {
	LayoutBox{margin_left: Distance::Absolute(0.),
		  margin_right: Distance::Absolute(0.),
		  margin_top: Distance::Absolute(0.),
		  margin_bottom: Distance::Absolute(0.),
		  padding_left: Distance::Absolute(0.),
		  padding_right: Distance::Absolute(0.),
		  padding_top: Distance::Absolute(0.),
		  padding_bottom: Distance::Absolute(0.),
		  width: None,
		  height: None,
		  visual_width: Distance::Absolute(0.),
		  visual_height: None,
		  content: Content::Solid([1.0, 1.0, 1.0, 0.0])}
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Content {
    Solid([f64;4]),
    Text(Label)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub text: String,
    pub font_color: [f64;4],
    pub weight: FontWeight,
    pub slant: FontSlant,
}
