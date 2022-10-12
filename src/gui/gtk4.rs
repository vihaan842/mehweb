use adw::prelude::*;
use adw::{ApplicationWindow, HeaderBar};
use adw::gtk::{Application, Orientation, Entry, DrawingArea};

use std::rc::Rc;
use std::cell::RefCell;

use crate::renderer::Doc;
use crate::gui::Gui;

pub struct Gtk4Gui {
    document: Rc<RefCell<Rc<Doc>>>,
    app: Application,
}

impl Gui for Gtk4Gui {
    fn new() -> Gtk4Gui {
	let app = Application::builder()
            .application_id("com.example.FirstAdwaitaApp")
            .build();
	app.connect_startup(move |_| {
            adw::init();
	});
	// used to render document
	let document: Rc<RefCell<Rc<Doc>>> = Rc::new(RefCell::new(Rc::new(Doc::Blank)));
	let return_document = Rc::clone(&document);
	// runs when app starts
	app.connect_activate(move |app| {
	    // holds header bar and browser engine
	    let content = adw::gtk::Box::new(Orientation::Vertical, 0);
            content.append(
		&HeaderBar::builder()
                    .title_widget(&adw::WindowTitle::new("MehWeb", ""))
                    .build(),
            );

	    // copy of document to be used in app
	    let document = Rc::clone(&document);
	    let document_setter = Rc::clone(&document);

	    let drawing_area = DrawingArea::new();
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

	    // the urlbar
	    let urlbar = Entry::new();
	    urlbar.connect_activate(move |entry| {
		let url = entry.buffer().text();
		let document = Rc::clone(&document_setter);
		let doc = crate::protocols::load_doc(url);
		doc.render();
		*document.borrow_mut() = doc;
		drawing_area.queue_draw();
	    });
	    content.append(&urlbar);
	    
	    
	    // window
	    let window = ApplicationWindow::builder()
		.application(app)
		.content(&content)
		.build();
	    window.show();
	});
	Gtk4Gui{document: return_document, app: app}
    }
    fn run(&self) {
	self.app.run();
    }
}
