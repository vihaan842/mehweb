mod http;
mod file;

use std::rc::Rc;

use crate::renderer::{Doc, web::{css, html}};

pub fn load_doc(url: String) -> Rc<Doc> {
    if url.starts_with("http://") {
	let http_return = http::load(url.trim_start_matches("http://"));
	println!("{}", http_return);

	// separate headers from body
	let (_headers, body) = http_return.split_once("\r\n\r\n").unwrap();
	
	// parse body to Doc
	let parsed_html = html::parse(body.to_string());
	let css = parsed_html.find_css();
	let parsed_css = css::parse(css);
	html::apply_css(parsed_css.clone(), Rc::clone(&parsed_html));
	println!("{}", parsed_html);
	return Rc::new(Doc::Web(parsed_html));
    } else {
	Rc::new(file::load(url.trim_start_matches("file://").to_string()))
    }
}
