/// Vectors
/// 
/// # Vec2, Vec3, Vec4 and associated functions
/// 
/// The structs do not have setters for the x, y, z, w internal data.
/// Everything is f32.
/// - TODO: make common implementations to traits instead
/// - TODO: overload common operators like + - * / for add, sub, mul, div, etc.

/// Vec2
#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// Vec3
#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Vec4
#[derive(Copy, Clone, Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}


impl Vec2 {
    /// Vec2 lenght
    /// 
    /// Returns lenght of the vector
    pub fn lenght(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    pub fn add(&self, v: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }

    pub fn sub(&self, v: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x - v.x,
            y: self.y - v.y,
        }
    }

    pub fn mul(&self, s: f32) -> Vec2 {
        Vec2 {
            x: self.x * s,
            y: self.y * s,
        }
    }

    pub fn div(&self, s: f32) -> Vec2 {
        let mut factor = s;
        if s == 0.0 {
            factor = 1.0;
        }

        Vec2 {
            x: self.x / factor,
            y: self.y / factor,
        }
    }

    pub fn dot(&self, v: &Vec2) -> f32 {
        ((self.x * v.x) + (self.y * v.y)) as f32
    }

    pub fn normalize(&mut self) {
        let le: f32 = ((self.x * self.x) + (self.y * self.y)).sqrt();
        self.x /= le;
        self.y /= le;
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::new()
    }
}

impl Vec3 {
    pub fn new() -> Self {
        Self { x: 0_f32, y: 0_f32, z: 0_f32 }
    }

    pub fn lenght(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn add(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    pub fn sub(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }

    pub fn mul(&self, s: f32) -> Vec3 {
        Vec3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }

    pub fn div(&self, s: f32) -> Vec3 {
        let mut factor = s;
        if s == 0.0 {
            factor = 1.0;
        }

        Vec3 {
            x: self.x / factor,
            y: self.y / factor,
            z: self.z / factor,
        }
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * v.z) - (self.z * v.y),
            y: (self.z * v.x) - (self.x * v.z),
            z: (self.x * v.y) - (self.y * v.x),
        }
    }

    pub fn tri_normal(a: &Vec3, b: &Vec3, c: &Vec3) -> Vec3 {
        let v1 = b.sub(a);
        let v2 = c.sub(a);
        v1.cross(&v2)
    }

    pub fn dot(&self, v: &Vec3) -> f32 {
        ((self.x * v.x) + (self.y * v.y) + (self.z * v.z)) as f32
    }

    pub fn normalize(&mut self) {
        let le: f32 = ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
        self.x /= le;
        self.y /= le;
        self.z /= le;
    }

    pub fn rotate_x(&mut self, angle: f32) {
        //self.y = self.y * angle.cos() - self.z * angle.sin();
        //self.z = self.y * angle.sin() + self.z * angle.cos();
        (self.y, self.z) = (
            self.y * angle.cos() - self.z * angle.sin(),
            self.y * angle.sin() + self.z * angle.cos()
        );
    }

    pub fn rotate_y(&mut self, angle: f32) {
        //self.x = self.x * angle.cos() - self.z * angle.sin();
        //self.z = self.x * angle.sin() + self.z * angle.cos();
        (self.x, self.z) = (
            self.x * angle.cos() - self.z * angle.sin(),
            self.x * angle.sin() + self.z * angle.cos()
        );
    }    

    pub fn rotate_z(&mut self, angle: f32) {
        //self.x = (self.x * angle.cos()) - (self.y * angle.sin());
        //self.y = (self.x * angle.sin()) + (self.y * angle.cos());
        //self.z = self.z;
        (self.x, self.y) = (
            (self.x * angle.cos()) - (self.y * angle.sin()),
            (self.x * angle.sin()) + (self.y * angle.cos())
        );
    }

    pub fn to_vec4(&self) -> Vec4 {
        Vec4 { x: self.x, y: self.y, z: self.z, w: 1.0_f32 }
    }



}

impl Vec4 {
    pub fn to_vec3(&self) -> Vec3 {
        Vec3 { x: self.x, y: self.y, z: self.z }
    }  
}