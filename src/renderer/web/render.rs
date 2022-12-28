use std::rc::Rc;
use cairo::{Context, Path, FontWeight, FontSlant, Glyph};
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
	    // create freetype library and face (for size info)
	    let lib = freetype::Library::init().unwrap();
	    let face = lib.new_face(crate::rules::DEFAULT_FONT, 0).unwrap();
	    // set size in cargo and freetype
	    let size = get_absolute_pos(height, label.font_size);
	    cr.set_font_size(size);
	    // placeholder dpi to look right. TODO: change
	    face.set_char_size(size as isize * 64, 0, 75, 0).unwrap();
	    //face.set_pixel_sizes(0, size as u32).unwrap();
	    // freetype units are 1/64ths of a pixel
	    let face_height = face.height() as f64 / 64.0;
	    let face_ascender = face.ascender() as f64 / 64.0;
	    // where we store our glyphs
	    let mut glyphs = Vec::new();
	    // the current position of the pen
	    let mut x = get_absolute_pos(width, left);
	    let mut y = get_absolute_pos(height, top) + face_ascender + face_height;
	    let mut prev_char: Option<char> = None;
	    for c in label.text.chars() {
		face.load_char(c as usize, freetype::face::LoadFlag::RENDER).unwrap();
		let ft_glyph = face.glyph();
		let glyph_metrics = ft_glyph.metrics();
		glyphs.push(Glyph::new(face.get_char_index(c as usize) as u64, x, y-face_height));
		x += glyph_metrics.horiAdvance as f64 / 64.0;
		if let Some(prev_c) = prev_char {
		    x += face.get_kerning(face.get_char_index(prev_c as usize), face.get_char_index(c as usize), freetype::face::KerningMode::KerningUnfitted).unwrap().x as f64 / 64.0;
		}
		if x > get_absolute_pos(width, render.visual_width) {
		    x = get_absolute_pos(width, left);
		    y += face_height;
		}
		prev_char = Some(c);
	    }
	    render.width = Some(Distance::Absolute(y));
	    render.height = Some(Distance::Absolute(y-get_absolute_pos(height, top)-face_ascender));
	    // return paths
	    cr.show_glyphs(&glyphs).expect("Invalid cairo surface state or path");
	    let color = label.font_color;
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
	    let mut margin_left = Distance::Absolute(0.);
	    let mut margin_right = Distance::Absolute(0.);
	    let mut margin_top = Distance::Absolute(0.);
	    let mut margin_bottom = Distance::Absolute(0.);
	    match node.css.borrow().get("margin") {
		Some(m) => {
		    let parts: Vec<&str> = m.split(" ").collect();
		    if parts.len() == 1 {
			margin_left = Distance::from(parts[0]);
			margin_right = Distance::from(parts[0]);
			margin_top = Distance::from(parts[0]);
			margin_bottom = Distance::from(parts[0]);
		    } else if parts.len() == 2 {
			margin_left = Distance::from(parts[1]);
			margin_right = Distance::from(parts[1]);
			margin_top = Distance::from(parts[0]);
			margin_bottom = Distance::from(parts[0]);
		    } else if parts.len() == 3 {
			margin_left = Distance::from(parts[1]);
			margin_right = Distance::from(parts[1]);
			margin_top = Distance::from(parts[0]);
			margin_bottom = Distance::from(parts[2]);
		    } else {
			margin_left = Distance::from(parts[3]);
			margin_right = Distance::from(parts[1]);
			margin_top = Distance::from(parts[0]);
			margin_bottom = Distance::from(parts[2]);
		    }
		},
		None => {},
	    };
	    match node.css.borrow().get("margin-left") {
		Some(m) => margin_left = Distance::from(m.to_string()),
		None => {}
	    };
	    match node.css.borrow().get("margin-right") {
		Some(m) => margin_right = Distance::from(m.to_string()),
		None => {}
	    };
	    match node.css.borrow().get("margin-top") {
		Some(m) => margin_top = Distance::from(m.to_string()),
		None => {}
	    };
	    match node.css.borrow().get("margin-bottom") {
		Some(m) => margin_bottom = Distance::from(m.to_string()),
		None => {}
	    };
	    let mut padding_left = Distance::Absolute(0.);
	    let mut padding_right = Distance::Absolute(0.);
	    let mut padding_top = Distance::Absolute(0.);
	    let mut padding_bottom = Distance::Absolute(0.);
	    match node.css.borrow().get("padding") {
		Some(m) => {
		    let parts: Vec<&str> = m.split(" ").collect();
		    if parts.len() == 1 {
			padding_left = Distance::from(parts[0]);
			padding_right = Distance::from(parts[0]);
			padding_top = Distance::from(parts[0]);
			padding_bottom = Distance::from(parts[0]);
		    } else if parts.len() == 2 {
			padding_left = Distance::from(parts[1]);
			padding_right = Distance::from(parts[1]);
			padding_top = Distance::from(parts[0]);
			padding_bottom = Distance::from(parts[0]);
		    } else if parts.len() == 3 {
			padding_left = Distance::from(parts[1]);
			padding_right = Distance::from(parts[1]);
			padding_top = Distance::from(parts[0]);
			padding_bottom = Distance::from(parts[2]);
		    } else {
			padding_left = Distance::from(parts[3]);
			padding_right = Distance::from(parts[1]);
			padding_top = Distance::from(parts[0]);
			padding_bottom = Distance::from(parts[2]);
		    }
		},
		None => {},
	    };
	    match node.css.borrow().get("padding-left") {
		Some(m) => padding_left = Distance::from(m.to_string()),
		None => {}
	    };
	    match node.css.borrow().get("padding-right") {
		Some(m) => padding_right = Distance::from(m.to_string()),
		None => {}
	    };
	    match node.css.borrow().get("padding-top") {
		Some(m) => padding_top = Distance::from(m.to_string()),
		None => {}
	    };
	    match node.css.borrow().get("padding-bottom") {
		Some(m) => padding_bottom = Distance::from(m.to_string()),
		None => {}
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
