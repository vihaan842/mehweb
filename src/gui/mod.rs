pub mod gtk4;

pub trait Gui {
    fn new() -> Self;
    fn run(&self);
}
