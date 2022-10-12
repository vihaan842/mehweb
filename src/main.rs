mod renderer;
mod gui;
mod rules;
mod protocols;

use crate::gui::{gtk4, Gui};

fn main() {
//     // sample html
//     let html = String::from(
// 	"
// <!DOCTYPE html>
// <html>
// <head>
// <style>
// body {
// /* sets colors */
// color: blue;
// background-color: red;
// /* position stuff */
// height: 100px;
// }
// body .cool {
// background-color: green;
// }
// </style>
// </head>
// <body>
// <h1 class='cool'>Title! ygygyg</h1>
// <p class='cool'>Paragraph! Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. </p>
// </body>
// </html>"
    //     );
    let gui = gtk4::Gtk4Gui::new();
    gui.run();
}
