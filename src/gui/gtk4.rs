use adw::prelude::*;
use adw::{ApplicationWindow, HeaderBar};
use adw::gtk::{Application, Orientation, DrawingArea};

use std::rc::Rc;
use std::cell::RefCell;

use crate::renderer::Doc;

pub fn build_window() -> (Box<dyn Fn()>, Box<dyn Fn(std::rc::Rc<crate::renderer::Doc>)>) {
    // create libadwaita app
    let app = Application::builder()
        .application_id("com.example.FirstAdwaitaApp")
        .build();
    app.connect_startup(move |_| {
        adw::init();
    });
    // used to render document
    let document: Rc<RefCell<Rc<Doc>>> = Rc::new(RefCell::new(Rc::new(Doc::Blank)));
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
	    // draw document
	    let document = Rc::clone(&document);
	    let paths = document.borrow().draw(cr, width, height);
	    for (path, color) in paths {
		cr.new_path();
		cr.set_source_rgba(color[0], color[1], color[2], color[3]);
		cr.append_path(&path);
		cr.fill().expect("Invalid cairo surface state or path");
	    }
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
    let render_document = move |doc: Rc<Doc>| {
	let document = Rc::clone(&document_setter);
	doc.render();
	*document.borrow_mut() = Rc::clone(&doc);
    };
    (Box::new(run_app), Box::new(render_document))
}
