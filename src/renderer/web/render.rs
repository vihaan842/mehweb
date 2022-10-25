use std::rc::Rc;
//use adw::gtk::{cairo::{Context, Path}, pango::{FontDescription, Style, Weight, Gravity, Stretch, Variant}};
use cairo::{Context, Path, Glyph, FontWeight, FontSlant, FontFace};
use crate::renderer::{layout::{Distance, Content, Label, Block}, web::html::{Node, NodeType}};

// recursive function to draw nodes
pub fn draw_node(cr: &Context, node: Rc<Node>, left: Distance, top: Distance, width: i32, height: i32) -> Vec<(Path, [f64;4])> {
    // don't draw display: none
    if node.css.borrow().get("display") == Some(&"none".to_string()) {
	return vec![(cr.copy_path().expect("Invalid cairo surface state or path"), [1.0, 1.0, 1.0, 0.0])];
    }
    let render = &mut *node.render.borrow_mut();
    match &mut render.content {
	// rectangles with solid color background
	Content::Solid(content) => {
	    // get to child nodes and figure out height
	    let mut child_height = Distance::Absolute(0.);
	    let mut child_paths = Vec::new();
	    let mut last_bottom_margin = Distance::Absolute(0.);
	    for child in node.children().borrow().iter() {
		match &mut child.render.borrow_mut().content {
		    Content::Solid(child_content) => {
			let mut top_margin = get_absolute_pos(height, child_content.margin_top)-get_absolute_pos(height, last_bottom_margin);
			if top_margin < 0. {
			    top_margin = 0.;
			}
			child_content.margin_top = Distance::Absolute(top_margin);
		    },
		    _ => {},
		}
		child_paths.append(&mut draw_node(cr, Rc::clone(&child), left+content.margin_left+content.padding_left, top+content.margin_top+content.padding_top+child_height, width, height));
		let child_render = &mut *child.render.borrow_mut();
		match &child_render.height {
		    Some(h) => {
			child_height += *h;
			last_bottom_margin = Distance::Absolute(0.);
			match &child_render.content {
			    Content::Solid(child_content) => {
				child_height += child_content.margin_top + child_content.margin_bottom;
				last_bottom_margin = child_content.margin_bottom;
			    },
			    _ => {},
			}
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
	    } + content.padding_top + content.padding_bottom;
	    let start_x = get_absolute_pos(width, left+content.margin_left);
	    let start_y = get_absolute_pos(height, top+content.margin_top);
	    let rect_width = get_absolute_pos(width, render.visual_width);
	    let rect_height = get_absolute_pos(height, visual_height);
	    cr.rectangle(start_x, start_y, rect_width, rect_height);
	    // return paths with current node at the bottom
	    let mut paths = vec![(cr.copy_path().expect("Invalid cairo surface state or path"), content.color)];
	    paths.append(&mut child_paths);
	    paths
	},
	// draw text
	Content::Text(label) => {
	    // // set font properties
	    // let mut font_desc = FontDescription::new();
	    // font_desc.set_family(crate::rules::DEFAULT_FONT);
	    // font_desc.set_absolute_size(get_absolute_pos(height, label.font_size) * adw::gtk::pango::SCALE as f64);
	    // font_desc.set_style(label.slant);
	    // font_desc.set_weight(label.weight);
	    // font_desc.set_gravity(Gravity::South);
	    // font_desc.set_stretch(Stretch::Normal);
	    // font_desc.set_variant(Variant::Normal);
	    // // make text with pango
	    // let layout = pangocairo::create_layout(&cr).unwrap();
	    // layout.set_font_description(Some(&font_desc));
	    // layout.set_width(get_absolute_pos(width, render.visual_width) as i32 * adw::gtk::pango::SCALE);
	    // layout.set_text(&label.text);
	    // // find out height and width of text
	    // let (text_width, text_height) = layout.pixel_size();
	    // render.height = Some(Distance::Absolute(text_height as f64));
	    // render.width = Some(Distance::Absolute(text_width as f64));
	    let lib = freetype::Library::init().unwrap();
	    let face = lib.new_face(crate::rules::DEFAULT_FONT, 0).unwrap();
	    cr.set_font_face(&FontFace::create_from_ft(&face).unwrap());
	    let size = get_absolute_pos(height, label.font_size);
	    cr.set_font_size(size);
	    let mut glyphs = Vec::new();
	    let mut x = 0.0;
	    let mut y = 0.0;
	    for c in label.text.chars() {
		glyphs.push(Glyph::new(face.get_char_index(c as usize) as u64, x*0.5*size+get_absolute_pos(width, left), y*size+get_absolute_pos(height, top)));
		x += 1.0;
		if x*0.5*size+get_absolute_pos(width, left) > get_absolute_pos(width, render.visual_width) {
		    x = 0.0;
		    y += 1.0;
		}
	    }
	    render.width = Some(Distance::Absolute(x*0.5*size));
	    render.height = Some(Distance::Absolute((y+1.0)*size));
	    // return paths
	    cr.show_glyphs(glyphs.as_slice()).expect("Invalid cairo surface state or path");
	    let color = label.font_color;
	    //pangocairo::layout_path(&cr, &layout);
	    vec![(cr.copy_path().expect("Invalid cairo surface state or path"), color)]
	},
    }
}

// gets the absolute position based on screen size
fn get_absolute_pos(size: i32, pos: Distance) -> f64 {
    match pos {
	Distance::Absolute(pixels) => pixels,
	Distance::Relative(percent) => percent * size as f64,
	Distance::Combo(pixels, percent) => pixels + percent * size as f64
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
		Some(c) => get_color(c.to_string()),
		None => [1.0, 1.0, 1.0, 0.0]
	    };
	    // set everything
	    let layout_box = &mut *node.render.borrow_mut();
	    let mut content = Block::new();
	    // weird condition where body's margin is actually padding
	    if tag_name == "body" {
		content.padding_left = margin_left;
		content.padding_right = margin_right;
		content.padding_top = margin_top;
		content.padding_bottom = margin_bottom;
		layout_box.visual_width = width + margin_left + margin_right;
	    } else {
		content.margin_left = margin_left;
		content.margin_right = margin_right;
		content.margin_top = margin_top;
		content.margin_bottom = margin_bottom;
		content.padding_left = padding_left;
		content.padding_right = padding_right;
		content.padding_top = padding_top;
		content.padding_bottom = padding_bottom;
		layout_box.visual_width = width;
	    }
	    layout_box.visual_height = height;
	    content.color = color;		
	    layout_box.content = Content::Solid(content);
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
		Some(c) => get_color(c.to_string()),
		None => [0.0, 0.0, 0.0, 1.0]
	    };
	    let font_size = match node.css.borrow().get("font-size") {
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
