use crate::vector::Vec2;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub v: Vec2,
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for Point {
    fn default() -> Self {
        Self::new()
    }
}

impl Point {
    pub fn new() -> Self {
        let v = Vec2 { x: 0.0, y: 0.0 };
        let a: u8 = 0xff;
        let r: u8 = 0x00;
        let g: u8 = 0x00;
        let b: u8 = 0x00;
        Self { v, a, r, g, b }
    }

    pub fn color4b(&self) -> u32 {
        u32::from_be_bytes([self.a, self.r, self.g, self.b]) //ARGB888
    }

    pub fn set_color4b(&mut self, color: u32) {
        let color: [u8; 4] = color.to_be_bytes();
        self.a = color[0];
        self.r = color[1];
        self.g = color[2];
        self.b = color[3];
    }
}
