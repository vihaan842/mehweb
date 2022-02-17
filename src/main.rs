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
/* sets background color to black */
background-color: black /*#000000*/;
/* position stuff */
height: 100px;
}
body .cool {
height: 10px;
width: 20px;
}
</style>
</head>
<body>
<p class='cool' style='background-color:red;color:green' onclick='console.log(\"fart\"); console.log(\"poo\")'>Some text</p>
<p class='cool' style='background-color:green;'>Some more text</p>
<p>Some text <a href='https://www.example.com/'>with a link</a></p>
<img src='fart.png' class='cool' style='background-color:blue;'>
</body>
</html>"
    );
    let parsed_html = html::parse(html);
    let css = parsed_html.find_css();
    let parsed_css = css::parse(css);
    html::apply_css(parsed_css.clone(), Rc::clone(&parsed_html));
    print!("{}", parsed_html);
    let mut window = graphics::Window::new();
    window.render_document(parsed_html);
    window.start()
}
