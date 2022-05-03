use adw::prelude::*;
use adw::{ApplicationWindow, HeaderBar};
use adw::gtk::{Application, Orientation, DrawingArea, cairo::{Context, Path}, pango::{FontDescription, Style, Weight, Gravity, Stretch, Variant}};

use std::rc::Rc;
use std::cell::RefCell;

use crate::layout::{Distance, Content, Label};
use crate::html::{Node, NodeType};

pub fn build_window() -> (Box<dyn Fn()>, Box<dyn Fn(std::rc::Rc<crate::html::Node>)>) {
    // create libadwaita app
    let app = Application::builder()
        .application_id("com.example.FirstAdwaitaApp")
        .build();
    app.connect_startup(move |_| {
        adw::init();
    });
    // used to render document
    let document: Rc<RefCell<Option<Rc<Node>>>> = Rc::new(RefCell::new(None));
    let document_setter = Rc::clone(&document);
    // runs when app starts
    app.connect_activate(move |app| {
	// holds header bar and browser engine
	let content = adw::gtk::Box::new(Orientation::Vertical, 0);
        content.append(
	    &HeaderBar::builder()
                .title_widget(&adw::WindowTitle::new("MehWeb", ""))
                .build(),
        );
	let drawing_area = DrawingArea::new();
	let document = Rc::clone(&document);
	// draws document
	drawing_area.set_draw_func(move |_, cr, width, height| {
	    // recursive function to draw nodes
	    fn draw_node(cr: &Context, node: Rc<Node>, left: Distance, top: Distance, width: i32, height: i32) -> Vec<(Path, [f64;4])> {
		// don't draw display: none
		if node.css.borrow().get("display") == Some(&"none".to_string()) {
		    return vec![(cr.copy_path().expect("Invalid cairo surface state or path"), [1.0, 1.0, 1.0, 0.0])];
		}
		let render = &mut *node.render.borrow_mut();
		match &render.content {
		    // rectangles with solid color background
		    Content::Solid(color) => {
			// get to child nodes and figure out height
			let mut child_height = Distance::Absolute(0.);
			let mut child_paths = Vec::new();
			let mut last_bottom_margin = Distance::Absolute(0.);
			for child in node.children().borrow().iter() {
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
			// draw rectangle
			let visual_height = match render.visual_height {
			    Some(h) => h,
			    None => child_height
			} + render.padding_top + render.padding_bottom;
			let start_x = get_absolute_pos(width, left+render.margin_left);
			let start_y = get_absolute_pos(height, top+render.margin_top);
			let rect_width = get_absolute_pos(width, render.visual_width);
			let rect_height = get_absolute_pos(height, visual_height);
			cr.rectangle(start_x, start_y, rect_width, rect_height);
			// return paths with current node at the bottom
			let mut paths = vec![(cr.copy_path().expect("Invalid cairo surface state or path"), *color)];
			paths.append(&mut child_paths);
			paths
		    },
		    // draw text
		    Content::Text(label) => {
			// set font properties
			let mut font_desc = FontDescription::new();
			font_desc.set_family(crate::rules::DEFAULT_FONT);
			font_desc.set_absolute_size(get_absolute_pos(height, label.font_size) * adw::gtk::pango::SCALE as f64);
			font_desc.set_style(label.slant);
			font_desc.set_weight(label.weight);
			font_desc.set_gravity(Gravity::South);
			font_desc.set_stretch(Stretch::Normal);
			font_desc.set_variant(Variant::Normal);
			// make text with pango
			let layout = pangocairo::create_layout(&cr).unwrap();
			layout.set_font_description(Some(&font_desc));
			layout.set_width(get_absolute_pos(width, render.visual_width) as i32 * adw::gtk::pango::SCALE);
			layout.set_text(&label.text);
			// find out height and width of text
			let (text_width, text_height) = layout.pixel_size();
			render.height = Some(Distance::Absolute(text_height as f64));
			render.width = Some(Distance::Absolute(text_width as f64));
			// return paths
			cr.move_to(get_absolute_pos(width, left), get_absolute_pos(height, top));
			let color = label.font_color;
			pangocairo::layout_path(&cr, &layout);
			vec![(cr.copy_path().expect("Invalid cairo surface state or path"), color)]
		    },
		}
	    }
	    // draw document
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
	// get drawing area that is at least 500x500
	drawing_area.set_size_request(500, 500);
	content.append(&drawing_area);
	// window
	let window = ApplicationWindow::builder()
	    .application(app)
	    .content(&content)
	    .build();
	window.show();
    });
    // functions to run app and render document
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

// gets the absolute position based on screen size
fn get_absolute_pos(size: i32, pos: crate::layout::Distance) -> f64 {
    match pos {
	crate::layout::Distance::Absolute(pixels) => pixels,
	crate::layout::Distance::Relative(percent) => percent * size as f64,
	crate::layout::Distance::Combo(pixels, percent) => pixels + percent * size as f64
    }
}

// gets color in rgba
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

// render nodes into boxes
pub fn render_node(node: Rc<Node>, max_width: Distance, max_height: Distance) {
    match &node.node_type {
	// document
	NodeType::Document(children) => {
	    // get to children
	    for child in children.borrow().iter() {
		render_node(Rc::clone(&child), max_width, max_height);
	    }
	},
	// containers
	NodeType::Container(tag_name, children, _) => {
	    // find margins and padding
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
	    // get width if specified
	    let width = match node.css.borrow().get("width") {
		Some(w) => Distance::from(w.to_string()),
		None => max_width-margin_left-margin_right-padding_left-padding_right,
	    };
	    // get height if specified
	    let height = match node.css.borrow().get("height") {
		Some(w) => Some(Distance::from(w.to_string())),
		None => None,
	    };
	    // get color if specified or transparent
	    let color = match node.css.borrow().get("background-color") {
		Some(c) => crate::graphics::get_color(c.to_string()),
		None => [1.0, 1.0, 1.0, 0.0]
	    };
	    // set everything
	    let layout_box = &mut *node.render.borrow_mut();
	    // weird condition where body's margin is actually padding
	    if tag_name == "body" {
		layout_box.padding_left = margin_left;
		layout_box.padding_right = margin_right;
		layout_box.padding_top = margin_top;
		layout_box.padding_bottom = margin_bottom;
		layout_box.visual_width = width + margin_left + margin_right;
	    } else {
		layout_box.margin_left = margin_left;
		layout_box.margin_right = margin_right;
		layout_box.margin_top = margin_top;
		layout_box.margin_bottom = margin_bottom;
		layout_box.padding_left = padding_left;
		layout_box.padding_right = padding_right;
		layout_box.padding_top = padding_top;
		layout_box.padding_bottom = padding_bottom;
		layout_box.visual_width = width;
	    }
	    layout_box.visual_height = height;
	    layout_box.content = Content::Solid(color);
	    let new_max_width = max_width-margin_left-margin_right-padding_left-padding_right;
	    let new_max_height = max_height-margin_top-margin_bottom-padding_top-padding_bottom;
	    // get to children
	    for child in children.borrow().iter() {
		render_node(Rc::clone(&child), new_max_width, new_max_height);
	    }
	},
	NodeType::Text(t) => {
	    // get font/text properties
	    let color = match node.css.borrow().get("color") {
		Some(c) => crate::graphics::get_color(c.to_string()),
		None => [0.0, 0.0, 0.0, 1.0]
	    };
	    let font_size = match node.css.borrow().get("font-size") {
		Some(s) => Distance::from(s.to_string()),
		None => Distance::Absolute(crate::rules::DEFAULT_FONT_SIZE as f64)
	    };
	    let weight = match node.css.borrow().get("font-weight") {
		Some(s) => match s.as_str() {
		    "bold" => Weight::Bold,
		    _ => Weight::Normal,
		},
		None => Weight::Normal,
	    };
	    let slant = match node.css.borrow().get("font-style") {
		Some(s) => match s.as_str() {
		    "italic" => Style::Italic,
		    "oblique" => Style::Oblique,
		    _ => Style::Normal,
		},
		None => Style::Normal,
	    };
	    // set label
	    let layout_box = &mut *node.render.borrow_mut();
	    layout_box.visual_width = max_width;
	    layout_box.content = Content::Text(Label{text: t.to_string(),
						     font_size: font_size,
						     font_color: color,
						     weight: weight,
						     slant: slant});
	},
    }
}
