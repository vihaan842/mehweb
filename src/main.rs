mod parser;
mod rules;

fn main() {
    // sample text
    let html = String::from(
	"<html>
<p onclick='console.log(\"fart\"); console.log(\"poo\")'>Some text</p>
<!--add this in later
<!--<p>Some more text</p>-->
<p>Some text <a href='https://www.example.com/'>with a link</a></p>
<img src='fart.png'>
</html>"
    );
    let parsed = parser::parse(html);
    for item in parsed {
	print!("{}", item);
    }
}
