use adw::prelude::*;
use adw::{ApplicationWindow, HeaderBar};
use adw::gtk::{Application, Orientation, DrawingArea, cairo::{FontSlant, FontWeight, Context, Path}};

use std::rc::Rc;
use std::cell::RefCell;

use crate::layout::{Distance, Content, Label};
use crate::html::{Node, NodeType};

pub fn build_window() -> (Box<dyn Fn()>, Box<dyn Fn(std::rc::Rc<crate::html::Node>)>) {
    let app = Application::builder()
        .application_id("com.example.FirstAdwaitaApp")
        .build();
    app.connect_startup(move |_| {
        adw::init();
    });
    let document: Rc<RefCell<Option<Rc<Node>>>> = Rc::new(RefCell::new(None));
    let document_setter = Rc::clone(&document);
    app.connect_activate(move |app| {
	let content = adw::gtk::Box::new(Orientation::Vertical, 0);
        content.append(
	    &HeaderBar::builder()
                .title_widget(&adw::WindowTitle::new("MehWeb", ""))
                .build(),
        );
	let drawing_area = DrawingArea::new();
	let document = Rc::clone(&document);
	drawing_area.set_draw_func(move |_, cr, width, height| {
	    // recursive function to draw nodes
	    fn draw_node(cr: &Context, node: Rc<Node>, left: Distance, top: Distance, width: i32, height: i32) -> Vec<(Path, [f64;4])> {
		if node.css.borrow().get("display") == Some(&"none".to_string()) {
		    return vec![(cr.copy_path().expect("Invalid cairo surface state or path"), [1.0, 1.0, 1.0, 0.0])];
		}
		let render = &mut *node.render.borrow_mut();
		match &render.content {
		    Content::Solid(color) => {
			// get to child nodes
			let mut child_height = Distance::Absolute(0.);
			let mut child_paths = Vec::new();
			let mut last_bottom_margin = Distance::Absolute(0.);
			for child in node.children.borrow().iter() {
			    let mut top_margin = get_absolute_pos(height, child.render.borrow().margin_top)-get_absolute_pos(height, last_bottom_margin);
			    if top_margin < 0. {
				top_margin = 0.;
			    }
			    child.render.borrow_mut().margin_top = Distance::Absolute(top_margin);
			    child_paths.append(&mut draw_node(cr, Rc::clone(&child), left+render.margin_left+render.padding_left, top+render.margin_top+render.padding_top+child_height, width, height));
			    let child_render = &mut *child.render.borrow_mut();
			    match &child_render.height {
				Some(h) => {
				    child_height += child_render.margin_top + *h + child_render.margin_bottom;
				    last_bottom_margin = child_render.margin_bottom;
				},
				None => {}
			    }
			    cr.new_path();
			}
			render.height = Some(child_height);
			let visual_height = match render.visual_height {
			    Some(h) => h,
			    None => child_height
			} + render.padding_top + render.padding_bottom;
			let start_x = get_absolute_pos(width, left+render.margin_left);
			let start_y = get_absolute_pos(height, top+render.margin_top);
			let rect_width = get_absolute_pos(width, render.visual_width);
			let rect_height = get_absolute_pos(height, visual_height);
			cr.rectangle(start_x, start_y, rect_width, rect_height);
			let mut paths = vec![(cr.copy_path().expect("Invalid cairo surface state or path"), *color)];
			paths.append(&mut child_paths);
			paths
		    },
		    Content::Text(label) => {
			let color = label.font_color;
			cr.select_font_face(crate::rules::DEFAULT_FONT, label.slant, label.weight);
			cr.set_font_size(get_absolute_pos(height, render.visual_height.unwrap()));
			let fe = cr.font_extents().expect("Invalid cairo surface state");
			let start_x = get_absolute_pos(width, left);
			let start_y = get_absolute_pos(height, top)-fe.descent+fe.height;
			cr.move_to(start_x, start_y);
			cr.text_path(&label.text);
			render.height = Some(Distance::Absolute(fe.height));
			vec![(cr.copy_path().expect("Invalid cairo surface state or path"), color)]
		    },
		}
	    }
	    let document = Rc::clone(&document);
	    match &*document.borrow() {
		Some(document) => {
		    let paths = draw_node(cr, Rc::clone(document), Distance::Absolute(0.), Distance::Absolute(0.), width, height);
		    for (path, color) in paths {
			cr.new_path();
			cr.set_source_rgba(color[0], color[1], color[2], color[3]);
			cr.append_path(&path);
			cr.fill().expect("Invalid cairo surface state or path");
		    }
		},
		None => {}
	    };
	});
	drawing_area.set_size_request(500, 500);
	content.append(&drawing_area);
	let window = ApplicationWindow::builder()
	    .application(app)
        // add content to window
	    .content(&content)
	    .build();
	window.show();
    });
    let run_app = move || {
	app.run();
    };
    let render_document = move |doc: Rc<Node>| {
	let document = Rc::clone(&document_setter);
	render_node(Rc::clone(&doc), Distance::Relative(1.), Distance::Relative(1.));
	*document.borrow_mut() = Some(Rc::clone(&doc));
    };
    (Box::new(run_app), Box::new(render_document))
}

fn get_absolute_pos(size: i32, pos: crate::layout::Distance) -> f64 {
    match pos {
	crate::layout::Distance::Absolute(pixels) => pixels,
	crate::layout::Distance::Relative(percent) => percent * size as f64,
	crate::layout::Distance::Combo(pixels, percent) => pixels + percent * size as f64
    }
}

pub fn get_color(color: String) -> [f64;4] {
    if color.starts_with("rgb") {
	let args = color.trim_start_matches("rgb(").trim_end_matches(")").split(",").collect::<Vec<&str>>();
	[args[0].trim().parse::<f64>().unwrap()/255.,
	 args[1].trim().parse::<f64>().unwrap()/255.,
	 args[2].trim().parse::<f64>().unwrap()/255.,
	 1.0]
    } else if color.starts_with("#") {
	let hex_color = color.trim_start_matches("#");
	let r = u32::from_str_radix(&hex_color[0..2], 16).unwrap() as f64;
        let g = u32::from_str_radix(&hex_color[2..4], 16).unwrap() as f64;
        let b = u32::from_str_radix(&hex_color[4..6], 16).unwrap() as f64;
	[r/255.,
	 g/255.,
	 b/255.,
	 1.0]
    } else {
	for (key, value) in crate::rules::DEFAULT_COLORS {
	    if key.to_string() == color {
		return get_color(value.to_string());
	    }
	}
	[0.0, 0.0, 0.0, 1.0]
    }
}

pub fn render_node(node: Rc<Node>, max_width: Distance, max_height: Distance) {
    match &node.node_type {
	NodeType::Document => {
	    for child in node.children.borrow().iter() {
		render_node(Rc::clone(&child), max_width, max_height);
	    }
	},
	NodeType::Container(_) => {
	    let margin = match node.css.borrow().get("margin") {
		Some(m) => Distance::from(m.to_string()),
		None => Distance::Absolute(0.)
	    };
	    let margin_left = match node.css.borrow().get("margin-left") {
		Some(m) => Distance::from(m.to_string()),
		None => margin
	    };
	    let margin_right = match node.css.borrow().get("margin-right") {
		Some(m) => Distance::from(m.to_string()),
		None => margin
	    };
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
	    let padding_right = match node.css.borrow().get("padding-right") {
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
	    let width = match node.css.borrow().get("width") {
		Some(w) => Distance::from(w.to_string()),
		None => max_width-margin_left-margin_right-padding_left-padding_right,
	    };
	    let height = match node.css.borrow().get("height") {
		Some(w) => Some(Distance::from(w.to_string())),
		None => None,
	    };
	    let color = match node.css.borrow().get("background-color") {
		Some(c) => crate::graphics::get_color(c.to_string()),
		None => [1.0, 1.0, 1.0, 0.0]
	    };
	    let layout_box = &mut *node.render.borrow_mut();
	    layout_box.margin_left = margin_left;
	    layout_box.margin_right = margin_right;
	    layout_box.margin_top = margin_top;
	    layout_box.margin_bottom = margin_bottom;
	    layout_box.padding_left = padding_left;
	    layout_box.padding_right = padding_right;
	    layout_box.padding_top = padding_top;
	    layout_box.padding_bottom = padding_bottom;
	    layout_box.visual_width = width;
	    layout_box.visual_height = height;
	    layout_box.content = Content::Solid(color);
	    let new_max_width = max_width-margin_left-margin_right-padding_left-padding_right;
	    let new_max_height = max_height-margin_top-margin_bottom-padding_top-padding_bottom;
	    for child in node.children.borrow().iter() {
		render_node(Rc::clone(&child), new_max_width, new_max_height);
	    }
	},
	NodeType::Text(t) => {
	    let color = match node.css.borrow().get("color") {
		Some(c) => crate::graphics::get_color(c.to_string()),
		None => [0.0, 0.0, 0.0, 1.0]
	    };
	    let height = match node.css.borrow().get("font-size") {
		Some(s) => Distance::from(s.to_string()),
		None => Distance::Absolute(crate::rules::DEFAULT_FONT_SIZE as f64)
	    };
	    let weight = match node.css.borrow().get("font-weight") {
		Some(s) => match s.as_str() {
		    "bold" => FontWeight::Bold,
		    _ => FontWeight::Normal,
		},
		None => FontWeight::Normal,
	    };
	    let slant = match node.css.borrow().get("font-style") {
		Some(s) => match s.as_str() {
		    "italic" => FontSlant::Italic,
		    "oblique" => FontSlant::Oblique,
		    _ => FontSlant::Normal,
		},
		None => FontSlant::Normal,
	    };
	    let layout_box = &mut *node.render.borrow_mut();
	    layout_box.visual_width = max_width;
	    layout_box.visual_height = Some(height);
	    layout_box.content = Content::Text(Label{text: t.to_string(),
						     font_color: color,
						     weight: weight,
						     slant: slant});
	},
    }
}
