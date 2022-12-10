use crate::display::Display;

pub trait Scene {
    fn update(&mut self, t: u32, display: &Display);
    fn render(&self, display: &mut Display);
}