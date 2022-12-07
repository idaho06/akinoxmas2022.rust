use crate::vector::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub v1: usize,
    pub v2: usize,
    pub v3: usize,
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub z: f32,
    pub normal: Vec3,
}

impl Default for Triangle {
    fn default() -> Self {
        Self::new()
    }
}


impl Triangle {
    pub fn new() -> Self {
        Self {
            v1: 0_usize,
            v2: 0_usize,
            v3: 0_usize,
            a: 0_u8,
            r: 0_u8,
            g: 0_u8,
            b: 0_u8,
            z: 0_f32,
            normal: Vec3::new(),
        }
    }


}