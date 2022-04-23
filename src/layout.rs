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

impl Distance {
    fn from_string(s: String) -> Distance {
	if s.contains('+') {
	    let mut total = Distance::Absolute(0.);
	    let parts = s.split('+');
	    for part in parts {
		total += Distance::from_string(part.trim().to_string());
	    }
	    total
	} else if s.ends_with("%") {
	    Distance::Relative(s.trim_end_matches('%').parse::<f64>().unwrap()/100.)
	} else {
	    Distance::Absolute(s.trim_end_matches("px").parse::<f64>().unwrap())
	}
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rect {
    pub x: Distance,
    pub y: Distance,
    pub width: Distance,
    pub height: Distance,
    pub color: [f64;3],
}

impl Rect {
    pub fn new(x: Distance, y: Distance, width: Distance, height: Distance, color: [f64;3]) -> Rect {
	return Rect{x: x, y: y, width: width, height: height, color: color};
    }
}

pub fn render_node(node: Rc<Node>, y_offset: Distance) -> (Vec<Rect>, Distance) {
    match &node.node_type {
	NodeType::Document => {
	    let mut rects = vec![Rect::new(Distance::Absolute(0.),
					   y_offset,
					   Distance::Relative(1.),
					   Distance::Relative(1.),
					   [1.0,1.0,1.0])];
	    let mut y_pos = y_offset;
	    for child in node.children.borrow().iter() {
		let (child_rects, child_y_size) = render_node(Rc::clone(child), y_pos);
		for rect in child_rects {
		    rects.push(rect);
		}
		y_pos += child_y_size;
	    }
	    (rects, Distance::Relative(1.))
	},
	NodeType::Container(_) => {
	    let mut rects = Vec::new();
	    let mut y_pos = y_offset;
	    for child in node.children.borrow().iter() {
		let (child_rects, child_y_size) = render_node(Rc::clone(child), y_pos);
		for rect in child_rects {
		    rects.push(rect);
		}
		y_pos += child_y_size;
	    }
	    let width = match node.css.borrow().get("width") {Some(w) => Distance::from_string(w.to_string()), None => Distance::Relative(1.)};
	    let height = match node.css.borrow().get("height") {Some(h) => Distance::from_string(h.to_string()), None => y_pos-y_offset};
	    let color = match node.css.borrow().get("background-color") {Some(c) => crate::graphics::get_color(c.to_string()), None => [1.0, 1.0, 1.0]};
	    let mut return_rects = vec![Rect::new(Distance::Absolute(0.), y_offset, width, height, color)];
	    for rect in rects {
		return_rects.push(rect);
	    }
	    (return_rects, height)
	},
	NodeType::Text(_) => (vec![Rect::new(Distance::Absolute(0.),
					     Distance::Absolute(0.),
					     Distance::Absolute(0.),
					     Distance::Absolute(0.),
					     [1.0, 1.0, 1.0])],
			      Distance::Absolute(0.))
    }
}
