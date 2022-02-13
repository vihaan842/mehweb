mod html;
mod css;
mod rules;

use std::rc::Rc;

fn main() {
    // sample html
    let html = String::from(
	"
<!DOCTYPE html>
<html>
<head>
<style>
* {
color: green;
}
body {
color: blue;
background-color: red;
}
body .cool {
color: red;
}
</style>
</head>
<body>
<p class='cool' style='color:yellow;' onclick='console.log(\\\"fart\\\"); console.log(\"poo\")'>Some text</p>
<!--add this in later
<!--<p>Some more text</p>-->
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
}
