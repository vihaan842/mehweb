use adw::prelude::*;
use adw::{ApplicationWindow, HeaderBar};
use adw::gtk::{Application, Orientation, DrawingArea, cairo::{FontSlant, FontWeight}};

use std::rc::Rc;
use std::cell::RefCell;

pub fn build_window() -> (Box<dyn Fn()>, Box<dyn Fn(std::rc::Rc<crate::html::Node>)>) {
    let rects_getter: Rc<RefCell<Vec<crate::layout::Rect>>> = Rc::new(RefCell::new(Vec::new()));
    let rects_setter = Rc::clone(&rects_getter);
    let app = Application::builder()
        .application_id("com.example.FirstAdwaitaApp")
        .build();
    app.connect_startup(move |_| {
        adw::init();
    });
    app.connect_activate(move |app| {
	let content = adw::gtk::Box::new(Orientation::Vertical, 0);
        content.append(
	    &HeaderBar::builder()
                .title_widget(&adw::WindowTitle::new("MehWeb", ""))
                .build(),
        );
	let rects_getter = Rc::clone(&rects_getter);
	let drawing_area = DrawingArea::new();
	drawing_area.set_draw_func(move |_, cr, width, height| {
	    let rects: std::cell::Ref<Vec<crate::layout::Rect>> = rects_getter.borrow();
	    let rects: &Vec<crate::layout::Rect> = rects.as_ref();
	    for rect in rects {
		// find relative position
		let start_x: f64 = get_absolute_pos(width, rect.x);
		let start_y: f64 = get_absolute_pos(height, rect.y);
		let rect_width: f64 = get_absolute_pos(width, rect.width);
		let rect_height: f64 = get_absolute_pos(width, rect.visual_height);
		match &rect.label {
		    // draw rectangle
		    None => {
			cr.set_source_rgba(rect.color[0], rect.color[1], rect.color[2], rect.color[3]);
			cr.rectangle(start_x, start_y, rect_width, rect_height);
			cr.fill().expect("Invalid cairo surface state");
		    },
		    // draw text
		    Some(s) => {
			cr.set_source_rgb(0.0, 0.0, 0.0);
			cr.select_font_face("DejaVu Sans", FontSlant::Normal, FontWeight::Normal);
			cr.set_font_size(rect_height);
			cr.move_to(start_x, start_y+rect_height);
			cr.show_text(s).expect("Invalid cairo surface state");
		    }
		}
	    }
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
    let render_document = move |document: Rc<crate::html::Node>| {
	let rects = crate::layout::render_node(document);
	let mut rects_setter = rects_setter.borrow_mut();
	for rect in rects {
	    rects_setter.push(rect);
	}
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
