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
/* this is applied to everything */
* {
color: green;
}
body {
/* sets font color to blue */
color: blue;
/* sets background color to black */
background-color: /*red*/ black;
}
body .cool {
color: red;
}
</style>
</head>
<body>
<p class='cool' style='color:yellow;' onclick='console.log(\\\"fart\\\"); console.log(\"poo\")'>Some text</p>
<p class='cool'>Some more text</p>
<p>Some text <a href='https://www.example.com/'>with a link</a></p>
<img src='fart.png' class='cool'>
</body>
</html>"
    );
    let parsed_html = html::parse(html);
    let css = parsed_html.find_css();
    let parsed_css = css::parse(css);
    html::apply_css(parsed_css.clone(), Rc::clone(&parsed_html));
    print!("{}", parsed_html);
    let rect1 = layout::Rect::new(layout::Position::Absolute(10.),
				  layout::Position::Absolute(0.),
				  layout::Position::Absolute(200.),
				  layout::Position::Relative(0.5),
				  [0.5, 0.0, 1.0]);
    let rect2 = layout::Rect::new(layout::Position::Absolute(100.),
				  layout::Position::Absolute(100.),
				  layout::Position::Absolute(200.),
				  layout::Position::Relative(0.5),
				  [0.0, 0.5, 1.0]);

    let mut window = graphics::Window::new();
    window.add_rect(rect1);
    window.add_rect(rect2);
    window.start()
}
