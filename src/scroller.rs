use std::collections::HashMap;

use sdl2::rect::Rect;

use crate::{display::Display, scene::{Scene, Sequence}};

pub struct Scroller {
    message: String,
    message_len: usize,
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
        display.add_sprite("font", "./assets/GraffittiFONT.png");
        let char_map = Self::create_char_map();
        let message = "¡Este es un mensaje de AkinoSoft al mundo!
 ¡AkinoPoder! Las demos de navidad mas cutres de la demoscene.
 ¿No conoces al grupo AkinoSoft? Somos los mejores coders, preventas,
 ilustradores y administradores de sistemas que han existido. 
 Somos ricos y guapos, como jodidas estrellas de rock.
 ¡FELIZ NAVIDAD CABRONES!
 Saludos a mis coleguitas Ikky, Po, Rx, Fabi, Lethe y Palo.
 Por las redes estan: @ruben3d, @itorres@xin.cat, @PalomoGarron,
 @frikivetusto, @Rheovarn, @Rober_dcm, @ouros, @LordTaishi, @gerarw,
 @TamL46, @Qetu, @Titonosfe, @Rivudo, @ikkante, @pedro_ru_mo, @kanotson,
 @DavidBokeh, @reemplazable. 
 Besitos a mis amores Marta, Raquel y Ester.
 Code: @idaho06 Gfx: @ikkante Music: Fabi
 --( Pulsa ESC para salir )--"
            .replace('\n', "");
        let message_len = message.chars().count();
        // Populate the string message
        Self {
            message,
            message_len,
            string_pos: 0,
            first_char_x: display.w_width() as f32,
            char_map,
            letter_positions: Vec::new(),
            speed: 640.0,
        }
    }

    fn create_char_map() -> HashMap<String, Rect> {
        let mut char_map: HashMap<String, Rect> = HashMap::new();

        char_map.insert(' '.to_string(), Rect::new(1539, 170, 70, 254));
        char_map.insert('!'.to_string(), Rect::new(98, 170, 66, 254));
        char_map.insert('"'.to_string(), Rect::new(306, 170, 104, 254));
        // char_map.insert('#'.to_string(), Rect::new(186, 20, 69, 100));
        // char_map.insert('$'.to_string(), Rect::new(275, 20, 57, 100));
        char_map.insert('%'.to_string(), Rect::new(495, 170, 178, 254));
        char_map.insert('&'.to_string(), Rect::new(757, 170, 121, 254));
        // char_map.insert('\''.to_string(), Rect::new(569, 20, 28, 100));
        char_map.insert('('.to_string(), Rect::new(1227, 170, 64, 254));
        char_map.insert(')'.to_string(), Rect::new(97, 469, 64, 254));
        // char_map.insert('*'.to_string(), Rect::new(743, 20, 57, 100));
        char_map.insert('+'.to_string(), Rect::new(280, 469, 116, 254));
        char_map.insert(','.to_string(), Rect::new(540, 469, 54, 254));
        char_map.insert('-'.to_string(), Rect::new(768, 469, 68, 254));
        char_map.insert('.'.to_string(), Rect::new(1026, 469, 42, 254));
        char_map.insert('/'.to_string(), Rect::new(1228, 469, 96, 254));
        char_map.insert('0'.to_string(), Rect::new(753, 3220, 147, 254));
        char_map.insert('1'.to_string(), Rect::new(1030, 3220, 66, 254));
        char_map.insert('2'.to_string(), Rect::new(1245, 3220, 92, 254));
        char_map.insert('3'.to_string(), Rect::new(1481, 3220, 94, 254));
        char_map.insert('4'.to_string(), Rect::new(1718, 3220, 94, 254));
        char_map.insert('5'.to_string(), Rect::new(67, 3535, 81, 254));
        char_map.insert('6'.to_string(), Rect::new(322, 3535, 81, 254));
        char_map.insert('7'.to_string(), Rect::new(548, 3535, 79, 254));
        char_map.insert('8'.to_string(), Rect::new(785, 3535, 78, 254));
        char_map.insert('9'.to_string(), Rect::new(1023, 3535, 85, 254));
        char_map.insert(':'.to_string(), Rect::new(91, 772, 54, 254));
        char_map.insert(';'.to_string(), Rect::new(323, 772, 61, 254));
        // char_map.insert('<'.to_string(), Rect::new(20, 260, 69, 100));
        char_map.insert('='.to_string(), Rect::new(540, 772, 71, 254));
        // char_map.insert('>'.to_string(), Rect::new(198, 260, 69, 100));
        char_map.insert('?'.to_string(), Rect::new(777, 772, 88, 254));
        char_map.insert('@'.to_string(), Rect::new(984, 772, 148, 254));
        char_map.insert('A'.to_string(), Rect::new(1218, 772, 147, 254));
        char_map.insert('B'.to_string(), Rect::new(1473, 772, 117, 254));
        char_map.insert('C'.to_string(), Rect::new(1706, 772, 100, 254));
        char_map.insert('D'.to_string(), Rect::new(26, 1084, 156, 254));
        char_map.insert('E'.to_string(), Rect::new(282, 1084, 147, 254));
        char_map.insert('F'.to_string(), Rect::new(512, 1084, 152, 254));
        char_map.insert('G'.to_string(), Rect::new(721, 1084, 165, 254));
        char_map.insert('H'.to_string(), Rect::new(979, 1084, 148, 254));
        char_map.insert('I'.to_string(), Rect::new(1227, 1084, 120, 254));
        char_map.insert('J'.to_string(), Rect::new(1424, 1084, 191, 254));
        char_map.insert('K'.to_string(), Rect::new(1700, 1084, 171, 254));
        char_map.insert('L'.to_string(), Rect::new(56, 1400, 117, 254));
        char_map.insert('M'.to_string(), Rect::new(248, 1400, 211, 254));
        char_map.insert('N'.to_string(), Rect::new(526, 1400, 113, 254));
        char_map.insert('O'.to_string(), Rect::new(741, 1400, 158, 254));
        char_map.insert('P'.to_string(), Rect::new(1008, 1400, 121, 254));
        char_map.insert('Q'.to_string(), Rect::new(1213, 1400, 186, 254));
        char_map.insert('R'.to_string(), Rect::new(1473, 1400, 164, 254));
        char_map.insert('S'.to_string(), Rect::new(1688, 1400, 155, 254));
        char_map.insert('T'.to_string(), Rect::new(33, 1703, 164, 254));
        char_map.insert('U'.to_string(), Rect::new(287, 1703, 124, 254));
        char_map.insert('V'.to_string(), Rect::new(526, 1703, 122, 254));
        char_map.insert('W'.to_string(), Rect::new(731, 1703, 171, 254));
        char_map.insert('X'.to_string(), Rect::new(980, 1703, 136, 254));
        char_map.insert('Y'.to_string(), Rect::new(1230, 1703, 101, 254));
        char_map.insert('Z'.to_string(), Rect::new(1433, 1703, 150, 254));
        // char_map.insert('['.to_string(), Rect::new(631, 500, 43, 100));
        char_map.insert('\\'.to_string(), Rect::new(1039, 170, 62, 254));
        // char_map.insert(']'.to_string(), Rect::new(769, 500, 43, 100));
        // char_map.insert('^'.to_string(), Rect::new(832, 500, 69, 100));
        char_map.insert('_'.to_string(), Rect::new(1263, 3535, 71, 254));
        // char_map.insert('`'.to_string(), Rect::new(20, 620, 57, 100));
        char_map.insert('a'.to_string(), Rect::new(1700, 1703, 120, 254));
        char_map.insert('b'.to_string(), Rect::new(47, 2013, 86, 254));
        char_map.insert('c'.to_string(), Rect::new(289, 2013, 87, 254));
        char_map.insert('d'.to_string(), Rect::new(530, 2013, 99, 254));
        char_map.insert('e'.to_string(), Rect::new(780, 2013, 89, 254));
        char_map.insert('f'.to_string(), Rect::new(1008, 2013, 73, 254));
        char_map.insert('g'.to_string(), Rect::new(1245, 2013, 91, 254));
        char_map.insert('h'.to_string(), Rect::new(1467, 2013, 93, 254));
        char_map.insert('i'.to_string(), Rect::new(1717, 2013, 71, 254));
        char_map.insert('j'.to_string(), Rect::new(45, 2332, 82, 254));
        char_map.insert('k'.to_string(), Rect::new(308, 2332, 94, 254));
        char_map.insert('l'.to_string(), Rect::new(547, 2332, 57, 254));
        char_map.insert('m'.to_string(), Rect::new(733, 2332, 135, 254));
        char_map.insert('n'.to_string(), Rect::new(990, 2332, 110, 254));
        char_map.insert('o'.to_string(), Rect::new(1257, 2332, 103, 254));
        char_map.insert('p'.to_string(), Rect::new(1485, 2332, 88, 254));
        char_map.insert('q'.to_string(), Rect::new(1692, 2332, 114, 254));
        char_map.insert('r'.to_string(), Rect::new(71, 2606, 74, 254));
        char_map.insert('s'.to_string(), Rect::new(309, 2606, 81, 254));
        char_map.insert('t'.to_string(), Rect::new(540, 2606, 89, 254));
        char_map.insert('u'.to_string(), Rect::new(777, 2606, 103, 254));
        char_map.insert('v'.to_string(), Rect::new(1015, 2606, 101, 254));
        char_map.insert('w'.to_string(), Rect::new(44, 2938, 126, 254));
        char_map.insert('x'.to_string(), Rect::new(305, 2938, 104, 254));
        char_map.insert('y'.to_string(), Rect::new(524, 2938, 120, 254));
        char_map.insert('z'.to_string(), Rect::new(771, 2938, 109, 254));
        // char_map.insert('{'.to_string(), Rect::new(20, 860, 57, 100));
        // char_map.insert('|'.to_string(), Rect::new(97, 860, 43, 100));
        // char_map.insert('}'.to_string(), Rect::new(160, 860, 57, 100));
        // char_map.insert('~'.to_string(), Rect::new(237, 860, 69, 100));
        char_map.insert('¡'.to_string(), Rect::new(1028, 2938, 42, 254));
        char_map.insert('¿'.to_string(), Rect::new(545, 3220, 84, 254));

        char_map
    }
}

impl Scene for Scroller {
    fn update(&mut self, t: u32, display: &Display, _scene: &Option<Sequence>) {
        self.letter_positions.clear(); //self.letter_positions.truncate(0);
        let mut letters = self.message.chars(); // iterator!

        // x calculation on speed and time
        let time_factor = (t as f32 / 1000.0) as f32;
        self.first_char_x -= self.speed * time_factor;
        let mut x = self.first_char_x;

        // if we are beyond the message, start again
        if self.string_pos >= self.message_len {
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
            _ = letters.nth(self.string_pos - 1).unwrap_or_else(|| {
                panic!("Failed to discard position {} in message", self.string_pos)
            });
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
            dst_rect.set_y(display.w_height() as i32 - src_rect.h);
            self.letter_positions.push((src_rect, dst_rect));
            x += src_rect.width() as f32;
        }
    }

    fn render(&self, display: &mut Display) {
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
}
