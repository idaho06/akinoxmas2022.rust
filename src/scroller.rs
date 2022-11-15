use crate::display::Display;

pub struct Scroller {
    message: String,
    pos: usize,
    // list of Rects of the characters in the font texture?
}


impl Scroller {
    pub fn new() -> Self {
        // load the png to texture in &display
        // Populate the string message
        Self {
            message: String::from("     abcdefghjiklmnopqrstuvwxyz     "),
            pos: 0,
        }
    }
    pub fn update(&mut self, t: u32, display: &Display) {
        // Calculate positions of current characters using time passed
    }

    pub fn render(&self, display: &mut Display) {
        // actually copying the texture/buffers to display 
        // apply effects? 
    }
}