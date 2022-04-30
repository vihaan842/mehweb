mod html;
mod css;
mod rules;
mod graphics;
mod layout;

use std::rc::Rc;

fn main() {
    // sample html
    let html = String::from(
	"
<!DOCTYPE html>
<html>
<head>
<style>
body {
/* sets font color to blue */
color: blue;
/* position stuff */
height: 100px;
}
/*body .cool {
background-color: green;
}*/
</style>
</head>
<body>
<h1 class='cool'>Title! ygygyg</h1>
<p class='cool'>Paragraph!</p>
</body>
</html>"
    );
    let parsed_html = html::parse(html);
    let css = parsed_html.find_css();
    let parsed_css = css::parse(css);
    html::apply_css(parsed_css.clone(), Rc::clone(&parsed_html));
    print!("{}", parsed_html);
    let (run_app, render_document) = crate::graphics::build_window();
    render_document(parsed_html);
    run_app();
}
