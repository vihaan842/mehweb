use std::rc::Rc;
use std::io::Read;
use std::fs::File;

use crate::renderer::{Doc, web::{css, html}};

pub fn load(path: String) -> Doc {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let parsed_html = html::parse(contents);
    let css = parsed_html.find_css();
    let parsed_css = css::parse(css);
    html::apply_css(parsed_css.clone(), Rc::clone(&parsed_html));
    println!("{}", parsed_html);
    return Doc::Web(parsed_html);
}
