use std::collections::HashMap;

use sdl2::rect::Rect;

use crate::display::Display;

pub struct Scroller {
    message: String,
    string_pos: usize,
    first_char_x: f32,
    // list of Rects of the characters in the font texture?
    char_map: HashMap<String, Rect>,
    letter_positions: Vec<(Rect, Rect)>,
    speed: f32,
}

impl Scroller {
    pub fn new(display: &mut Display) -> Self {
        // load the png to texture in &mut display
        display.add_sprite("font", "./assets/retrolaserfont.png");
        let char_map = Self::create_char_map();
        // Populate the string message
        Self {
            message: "Este es un mensaje de AkinoSoft al mundo!
 AkinoPower! Las demos de navidad mas cutres de la demoscene.
 No conoces al grupo AkinoSoft? Somos los mejores coders, preventas
 y administradores de sistemas que ha existido. Somos ricos y guapos,
 como jodidas estrellas de rock.
 FELIZ NAVIDAD CABRONES!
 Saludos a mis coleguitas Ikky, Palo, Ruben3D, Rx, Lethe.
 Besitos a mis amores Marta, Raquel y Ester."
                .replace('\n', ""),
            string_pos: 0,
            first_char_x: display.w_width() as f32,
            char_map,
            letter_positions: Vec::new(),
            speed: 640.0,
        }
    }
    pub fn update(&mut self, t: u32, display: &Display) {
        self.letter_positions.clear(); //self.letter_positions.truncate(0);
        let mut letters = self.message.chars(); // iterator!

        // x calculation on speed and time
        let time_factor = (t as f32 / 1000.0) as f32;
        self.first_char_x -= self.speed * time_factor;
        let mut x = self.first_char_x;

        // if we are beyond the message, start again
        if self.string_pos >= self.message.len() {
            self.string_pos = 0;
            self.first_char_x = display.w_width() as f32;
            x = self.first_char_x;
        }
        // get first letter rect
        let first_letter = self.message.chars().nth(self.string_pos).unwrap_or(' ');
        // clippy warning: using `clone` on type `display::sdl2::rect::Rect` which implements the `Copy` trait
        //let first_letter_rect = self.char_map.get(&first_letter.to_string()).unwrap().clone();
        let first_letter_rect = *self
            .char_map
            .get(&first_letter.to_string())
            .unwrap_or_else(|| panic!("Rect not found by char: {}", first_letter));

        // check if first letter is beyond the screen
        if (x + first_letter_rect.width() as f32) < 0.0 {
            self.string_pos += 1;
            self.first_char_x = x + first_letter_rect.width() as f32;
            x = self.first_char_x;
        }

        if self.string_pos > 0 {
            // discard all previous letters
            _ = letters.nth(self.string_pos - 1).unwrap();
        }
        'message: loop {
            // calculate letter rects for font and screen
            if x > display.w_width() as f32 {
                break 'message;
            }
            let letter = letters.next().unwrap_or(' ');
            // clippy warning: use of `expect` followed by a function call
            //let src_rect = self.char_map.get(&letter.to_string()).expect(&format!("Rect not found by char: {}", letter)).clone();
            //clippy warning: using `clone` on type `display::sdl2::rect::Rect` which implements the `Copy` trait
            //let src_rect = self.char_map.get(&letter.to_string()).unwrap_or_else(|| panic!("Rect not found by char: {}", letter)).clone();
            let src_rect = *self
                .char_map
                .get(&letter.to_string())
                .unwrap_or_else(|| panic!("Rect not found by char: {}", letter));
            //let mut dst_rect = src_rect.clone(); // clippy warning: using `clone` on type `display::sdl2::rect::Rect` which implements the `Copy` trait
            let mut dst_rect = src_rect;
            dst_rect.set_x(x.round() as i32);
            dst_rect.set_y(display.w_height() as i32 - 100_i32);
            self.letter_positions.push((src_rect, dst_rect));
            x += src_rect.width() as f32;
        }
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
        for (src_rect, dst_rect) in &self.letter_positions {
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
        char_map.insert('H'.to_string(), Rect::new(20, 380, 67, 100));
        char_map.insert('I'.to_string(), Rect::new(107, 380, 44, 100));
        char_map.insert('J'.to_string(), Rect::new(171, 380, 47, 100));
        char_map.insert('K'.to_string(), Rect::new(238, 380, 66, 100));
        char_map.insert('L'.to_string(), Rect::new(324, 380, 53, 100));
        char_map.insert('M'.to_string(), Rect::new(397, 380, 76, 100));
        char_map.insert('N'.to_string(), Rect::new(493, 380, 68, 100));
        char_map.insert('O'.to_string(), Rect::new(581, 380, 68, 100));
        char_map.insert('P'.to_string(), Rect::new(669, 380, 59, 100));
        char_map.insert('Q'.to_string(), Rect::new(748, 380, 68, 100));
        char_map.insert('R'.to_string(), Rect::new(836, 380, 66, 100));
        char_map.insert('S'.to_string(), Rect::new(922, 380, 57, 100));
        char_map.insert('T'.to_string(), Rect::new(20, 500, 60, 100));
        char_map.insert('U'.to_string(), Rect::new(100, 500, 65, 100));
        char_map.insert('V'.to_string(), Rect::new(185, 500, 67, 100));
        char_map.insert('W'.to_string(), Rect::new(272, 500, 94, 100));
        char_map.insert('X'.to_string(), Rect::new(386, 500, 65, 100));
        char_map.insert('Y'.to_string(), Rect::new(471, 500, 75, 100));
        char_map.insert('Z'.to_string(), Rect::new(556, 500, 55, 100));
        char_map.insert('['.to_string(), Rect::new(631, 500, 43, 100));
        char_map.insert('\\'.to_string(), Rect::new(694, 500, 55, 100));
        char_map.insert(']'.to_string(), Rect::new(769, 500, 43, 100));
        char_map.insert('^'.to_string(), Rect::new(832, 500, 69, 100));
        char_map.insert('_'.to_string(), Rect::new(921, 500, 63, 100));
        char_map.insert('`'.to_string(), Rect::new(20, 620, 57, 100));
        char_map.insert('a'.to_string(), Rect::new(97, 620, 53, 100));
        char_map.insert('b'.to_string(), Rect::new(170, 620, 56, 100));
        char_map.insert('c'.to_string(), Rect::new(246, 620, 47, 100));
        char_map.insert('d'.to_string(), Rect::new(313, 620, 56, 100));
        char_map.insert('e'.to_string(), Rect::new(389, 620, 53, 100));
        char_map.insert('f'.to_string(), Rect::new(462, 620, 38, 100));
        char_map.insert('g'.to_string(), Rect::new(520, 620, 56, 100));
        char_map.insert('h'.to_string(), Rect::new(596, 620, 57, 100));
        char_map.insert('i'.to_string(), Rect::new(673, 620, 27, 100));
        char_map.insert('j'.to_string(), Rect::new(720, 620, 37, 100));
        char_map.insert('k'.to_string(), Rect::new(777, 620, 56, 100));
        char_map.insert('l'.to_string(), Rect::new(853, 620, 27, 100));
        char_map.insert('m'.to_string(), Rect::new(900, 620, 85, 100));
        char_map.insert('n'.to_string(), Rect::new(20, 740, 57, 100));
        char_map.insert('o'.to_string(), Rect::new(97, 740, 55, 100));
        char_map.insert('p'.to_string(), Rect::new(172, 740, 56, 100));
        char_map.insert('q'.to_string(), Rect::new(248, 740, 56, 100));
        char_map.insert('r'.to_string(), Rect::new(324, 740, 40, 100));
        char_map.insert('s'.to_string(), Rect::new(384, 740, 47, 100));
        char_map.insert('t'.to_string(), Rect::new(451, 740, 38, 100));
        char_map.insert('u'.to_string(), Rect::new(509, 740, 57, 100));
        char_map.insert('v'.to_string(), Rect::new(586, 740, 57, 100));
        char_map.insert('w'.to_string(), Rect::new(663, 740, 82, 100));
        char_map.insert('x'.to_string(), Rect::new(765, 740, 58, 100));
        char_map.insert('y'.to_string(), Rect::new(843, 740, 57, 100));
        char_map.insert('z'.to_string(), Rect::new(917, 740, 48, 100));
        char_map.insert('{'.to_string(), Rect::new(20, 860, 57, 100));
        char_map.insert('|'.to_string(), Rect::new(97, 860, 43, 100));
        char_map.insert('}'.to_string(), Rect::new(160, 860, 57, 100));
        char_map.insert('~'.to_string(), Rect::new(237, 860, 69, 100));

        char_map
    }
}
