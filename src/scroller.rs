use std::collections::HashMap;

use sdl2::rect::Rect;

use crate::display::Display;

pub struct Scroller {
    message: String,
    string_pos: usize,
    first_char_x: i32,
    // list of Rects of the characters in the font texture?
    char_map: HashMap<String, Rect>,
    letter_positions: Vec<(Rect, Rect)>,
}

impl Scroller {
    pub fn new(display: &mut Display) -> Self {
        // load the png to texture in &mut display
        display.add_sprite("font", "./assets/retrolaserfont.png");
        let char_map = Self::create_char_map();
        // Populate the string message
        Self {
            message: String::from("ABC 123 !\"$   ABABABABBABABA BBABABABAB BABABABABABABBABA BABABABABBABABABAB BABABABABBABABABA  "),
            string_pos: 0,
            first_char_x: display.w_width() as i32,
            char_map,
            letter_positions: Vec::new(),
        }
    }
    pub fn update(&mut self, t: u32, display: &Display) {
        // Calculate positions of current characters using time passed
        self.letter_positions.clear(); //self.letter_positions.truncate(0);
        let mut letters = self.message.chars();
        let mut x = self.first_char_x;
        'message: loop {
            if x > display.w_width() as i32 {
                break 'message;
            }
            let letter = letters.next().unwrap_or(' ');
            let src_rect = self.char_map.get(&letter.to_string()).unwrap().clone();
            let mut dst_rect = src_rect.clone();
            dst_rect.set_x(x);
            dst_rect.set_y(display.w_height() as i32 / 2);
            self.letter_positions.push((src_rect,dst_rect));
            x += src_rect.width() as i32;
        }
        self.first_char_x -= 1;
    }

    pub fn render(&self, display: &mut Display) {
        // actually copying the texture/buffers to display
        // apply effects?
        /* 
        let mut letters = self.message.chars();
        let mut x = self.first_char_x;
        'message: loop {
            if x > display.t_width() as i32 {
                break 'message;
            }
            let letter = letters.next().unwrap_or(' ');
            let rect = self.char_map.get(&letter.to_string()).unwrap();
            display.put_sprite_rect("font", x, display.w_height() as i32 / 2, rect);
            x += rect.width() as i32;
        }
        */
        for (src_rect,dst_rect) in &self.letter_positions {
            display.put_sprite_rect_rect("font", src_rect, dst_rect);
        }
    }

    fn create_char_map() -> HashMap<String, Rect> {
        let mut char_map: HashMap<String, Rect> = HashMap::new();

        char_map.insert(' '.to_string(), Rect::new(20, 20, 27, 100));
        char_map.insert('!'.to_string(), Rect::new(67, 20, 32, 100));
        char_map.insert('"'.to_string(), Rect::new(119, 20, 47, 100));
        char_map.insert('#'.to_string(), Rect::new(186, 20, 69, 100));
        char_map.insert('$'.to_string(), Rect::new(275, 20, 57, 100));
        char_map.insert('%'.to_string(), Rect::new(352, 20, 102, 100));
        char_map.insert('&'.to_string(), Rect::new(474, 20, 75, 100));
        char_map.insert('\''.to_string(), Rect::new(569, 20, 28, 100));
        char_map.insert('('.to_string(), Rect::new(617, 20, 43, 100));
        char_map.insert(')'.to_string(), Rect::new(680, 20, 43, 100));
        char_map.insert('*'.to_string(), Rect::new(743, 20, 57, 100));
        char_map.insert('+'.to_string(), Rect::new(820, 20, 69, 100));
        char_map.insert(','.to_string(), Rect::new(909, 20, 29, 100));
        char_map.insert('-'.to_string(), Rect::new(958, 20, 38, 100));
        char_map.insert('.'.to_string(), Rect::new(20, 140, 29, 100));
        char_map.insert('/'.to_string(), Rect::new(69, 140, 55, 100));
        char_map.insert('0'.to_string(), Rect::new(144, 140, 57, 100));
        char_map.insert('1'.to_string(), Rect::new(221, 140, 57, 100));
        char_map.insert('2'.to_string(), Rect::new(298, 140, 57, 100));
        char_map.insert('3'.to_string(), Rect::new(375, 140, 57, 100));
        char_map.insert('4'.to_string(), Rect::new(452, 140, 57, 100));
        char_map.insert('5'.to_string(), Rect::new(529, 140, 57, 100));
        char_map.insert('6'.to_string(), Rect::new(606, 140, 57, 100));
        char_map.insert('7'.to_string(), Rect::new(683, 140, 57, 100));
        char_map.insert('8'.to_string(), Rect::new(760, 140, 57, 100));
        char_map.insert('9'.to_string(), Rect::new(837, 140, 57, 100));
        char_map.insert(':'.to_string(), Rect::new(914, 140, 32, 100));
        char_map.insert(';'.to_string(), Rect::new(966, 140, 32, 100));
        char_map.insert('<'.to_string(), Rect::new(20, 260, 69, 100));
        char_map.insert('='.to_string(), Rect::new(109, 260, 69, 100));
        char_map.insert('>'.to_string(), Rect::new(198, 260, 69, 100));
        char_map.insert('?'.to_string(), Rect::new(287, 260, 49, 100));
        char_map.insert('@'.to_string(), Rect::new(356, 260, 77, 100));
        char_map.insert('A'.to_string(), Rect::new(453, 260, 68, 100));
        char_map.insert('B'.to_string(), Rect::new(541, 260, 61, 100));
        char_map.insert('C'.to_string(), Rect::new(622, 260, 58, 100));
        char_map.insert('D'.to_string(), Rect::new(700, 260, 66, 100));
        char_map.insert('E'.to_string(), Rect::new(786, 260, 55, 100));
        char_map.insert('F'.to_string(), Rect::new(861, 260, 52, 100));
        char_map.insert('G'.to_string(), Rect::new(933, 260, 65, 100));

        char_map
    }
}
