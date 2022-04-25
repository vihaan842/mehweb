use std::rc::Rc;
use crate::html::{Node, NodeType};

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
	if s.contains('+') {
	    let mut total = Distance::Absolute(0.);
	    let parts = s.split('+');
	    for part in parts {
		total += Distance::from(part.trim().to_string());
	    }
	    total
	} else if s.ends_with("%") {
	    Distance::Relative(s.trim_end_matches('%').parse::<f64>().unwrap()/100.)
	} else {
	    Distance::Absolute(s.trim_end_matches("px").parse::<f64>().unwrap())
	}
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pub x: Distance,
    pub y: Distance,
    pub width: Distance,
    pub height: Distance,
    pub visual_height: Distance,
    pub color: [f64;4],
    pub label: Option<String>,
}

impl Rect {
    pub fn new(x: Distance, y: Distance, width: Distance, height: Distance, visual_height: Distance, color: [f64;4]) -> Rect {
	return Rect{x: x, y: y, width: width, height: height, visual_height: visual_height, color: color, label: None};
    }
    pub fn new_with_label(color: [f64;4], font_size: usize, label: String) -> Rect {
	let height = Distance::Absolute(font_size as f64);
	return Rect{x: Distance::Absolute(0.), y: Distance::Absolute(0.), width: Distance::Relative(1.), height: height, visual_height: height, color: color, label: Some(label)};
    }
}

pub fn render_node(node: Rc<Node>) -> Vec<Rect> {
    match &node.node_type {
	NodeType::Document => {
	    let mut rects = vec![Rect::new(Distance::Absolute(0.),
					   Distance::Absolute(0.),
					   Distance::Relative(1.),
					   Distance::Relative(1.),
					   Distance::Relative(1.),
					   [1.0,1.0,1.0, 0.0])];
	    let mut y_pos = Distance::Absolute(0.);
	    for child in node.children.borrow().iter() {
		let child_rects = render_node(Rc::clone(child));
		if child_rects.len() > 0 {
		    let child_y_size = child_rects[0].height;
		    for mut rect in child_rects {
			rect.y += y_pos;
			rects.push(rect);
		    }
		    y_pos += child_y_size;
		}
	    }
	    rects
	},
	NodeType::Container(tag_name) => {
	    if tag_name == "style" {
		return Vec::new();
	    }
	    let mut rects = Vec::new();
	    let margin = match node.css.borrow().get("margin") {
		Some(m) => Distance::from(m.to_string()),
		None => Distance::Absolute(0.)
	    };
	    let margin_left = match node.css.borrow().get("margin-left") {
		Some(m) => Distance::from(m.to_string()),
		None => margin
	    };
	    // let margin_right = match node.css.borrow().get("margin-right") {
	    // 	Some(m) => {
	    // 	    Distance::from(m.to_string())
	    // 	},
	    // 	None => margin
	    // }
	    let margin_top = match node.css.borrow().get("margin-top") {
		Some(m) => Distance::from(m.to_string()),
		None => margin
	    };
	    let margin_bottom = match node.css.borrow().get("margin-bottom") {
		Some(m) => Distance::from(m.to_string()),
		None => margin
	    };
	    let padding = match node.css.borrow().get("padding") {
		Some(p) => Distance::from(p.to_string()),
		None => Distance::Absolute(0.),
	    };
	    let padding_left = match node.css.borrow().get("padding-left") {
		Some(p) => Distance::from(p.to_string()),
		None => padding,
	    };
	    let padding_top = match node.css.borrow().get("padding-top") {
		Some(p) => Distance::from(p.to_string()),
		None => padding,
	    };
	    let padding_bottom = match node.css.borrow().get("padding-bottom") {
		Some(p) => Distance::from(p.to_string()),
		None => padding,
	    };
	    let mut y_pos = margin_top+padding_top;
	    for child in node.children.borrow().iter() {
		let child_rects = render_node(Rc::clone(child));
		if child_rects.len() > 0 {
		    let child_y_size = child_rects[0].height;
		    for mut rect in child_rects {
			rect.y += y_pos;
			rect.x += margin_left + padding_left;
			rects.push(rect);
		    }
		    y_pos += child_y_size;
		}
	    }
	    y_pos += padding_bottom + margin_bottom;
	    let width = match node.css.borrow().get("width") {
		Some(w) => Distance::from(w.to_string()),
		None => Distance::Relative(1.)
	    };
	    let (height, visual_height) = match node.css.borrow().get("height") {
		Some(h) => {
		    let h = Distance::from(h.to_string());
		    if h > y_pos {
			(h, h)
		    } else {
			(y_pos, h)
		    }
		},
		None => (y_pos, y_pos-margin_top-margin_bottom)
	    };
	    let color = match node.css.borrow().get("background-color") {
		Some(c) => crate::graphics::get_color(c.to_string()),
		None => [1.0, 1.0, 1.0, 0.0]
	    };
	    let mut return_rects = vec![Rect::new(margin_left, margin_top, width, height, visual_height, color)];
	    for rect in rects {
		return_rects.push(rect);
	    }
	    return_rects
	},
	NodeType::Text(text) => {
	    let color = match node.css.borrow().get("color") {
		Some(c) => crate::graphics::get_color(c.to_string()),
		None => [0.0, 0.0, 0.0, 1.0]
	    };
	    let height = match node.css.borrow().get("font-size") {
		Some(s) => s.parse::<usize>().unwrap(),
		None => 10
	    };
	    vec![Rect::new_with_label(color,
				      height,
				      text.to_string())]
	}
    }
}
