pub mod layout;
pub mod web;

use std::rc::Rc;
use layout::Distance;

use cairo::{Path, Context};

pub enum Doc {
    Blank,
    Web(Rc<web::html::Node>),
}

impl Doc {
    pub fn draw(&self, cr: &Context, width: i32, height: i32) -> Vec<(Path, [f64;4])> {
	match &self {
	    Doc::Blank => Vec::new(),
	    Doc::Web(node) => web::render::draw_node(cr, Rc::clone(node), Distance::Absolute(0.), Distance::Absolute(0.), width, height),
	}
    }
    pub fn render(&self) {
	match &self {
	    Doc::Blank => {},
	    Doc::Web(node) => web::render::render_node(Rc::clone(&node), Distance::Relative(1.), Distance::Relative(1.)),
	}
    }
}
