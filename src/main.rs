mod renderer;
mod gui;
mod rules;

use std::rc::Rc;

use crate::renderer::{Doc, web::{css, html}};
use crate::gui::gtk4;

fn main() {
    // sample html
    let html = String::from(
	"
<!DOCTYPE html>
<html>
<head>
<style>
body {
/* sets colors */
color: blue;
background-color: red;
/* position stuff */
height: 100px;
}
body .cool {
background-color: green;
}
</style>
</head>
<body>
<h1 class='cool'>Title! ygygyg</h1>
<p class='cool'>Paragraph! Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. </p>
</body>
</html>"
    );
    let parsed_html = html::parse(html);
    let css = parsed_html.find_css();
    let parsed_css = css::parse(css);
    html::apply_css(parsed_css.clone(), Rc::clone(&parsed_html));
    print!("{}", parsed_html);
    let (run_app, render_document) = gtk4::build_window();
    render_document(Rc::new(Doc::Web(parsed_html)));
    run_app();
}
