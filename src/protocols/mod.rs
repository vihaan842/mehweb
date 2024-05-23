mod http;
mod file;

use std::rc::Rc;

use crate::renderer::{mtk::widgets::Content, web::{css, html, render}};

pub fn load_and_render_doc(url: String) -> Box<dyn Content> {
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
	let r = render::render_node(parsed_html);
	println!("{:?}", r);
	r
    } else {
	render::render_node(file::load(url.trim_start_matches("file://").to_string()))
    }
}
