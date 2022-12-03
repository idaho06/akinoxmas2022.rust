use crate::{display::Display, point::Point, vector::Vec3};
use rand::Rng;

pub struct Starfield {
    stars: Vec<Vec3>,
    limits: (f32, f32),
    screen_stars: Vec<Point>,
    direction: Vec3,
}

/* impl Default for Starfield {
    fn default() -> Self {
        Self::new()
    }
} */

impl Starfield {
    pub fn new(display: &mut Display) -> Self {
        // Create a vector of random Vec3 in space between (-1.0, -1.0, -1,0) to (1.0, 1.0, 1.0)
        let num_stars: usize = 2000;
        let zero_vec = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut stars = vec![zero_vec; num_stars];
        let mut rng = rand::thread_rng();
        let limits = (-1.0, 1.0);

        for star in stars.iter_mut() {
            star.x = rng.gen_range(limits.0..limits.1);
            star.y = rng.gen_range(limits.0..limits.1);
            star.z = rng.gen_range(limits.0..limits.1);
        }

        display.add_streaming_buffer("starfield", 640, 360);

        //println!("{:?}", stars);

        // clippy warning: redundant field names in struct initialization
        Self {
            //stars: stars,
            //limits: limits,
            stars,
            limits,
            screen_stars: Vec::<Point>::new(),
            direction: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
        }
    }

    pub fn stars(&self) -> &Vec<Vec3> {
        &self.stars
    }

    pub fn displace(&mut self, v: &Vec3) {
        for star in self.stars.iter_mut() {
            *star = star.add(v);
            if star.x > self.limits.1 {
                star.x = self.limits.0 + star.x.fract();
            }
            if star.x < self.limits.0 {
                star.x = self.limits.1 - star.x.fract();
            }
            if star.y > self.limits.1 {
                star.y = self.limits.0 + star.y.fract();
            }
            if star.y < self.limits.0 {
                star.y = self.limits.1 - star.y.fract();
            }
            if star.z > self.limits.1 {
                star.z = self.limits.0 + star.z.fract();
            }
            if star.z < self.limits.0 {
                star.z = self.limits.1 - star.z.fract();
            }
        }
        //println!("{:?}", self.stars);
    }

    fn posterize(&self, min: f32, max: f32, value: f32) -> u8 {
        if value < min {
            return 0xff;
        }
        if value > max {
            return 0x00;
        }
        let delta = max - min;
        let value = value - min;
        let value = value / delta; // this should return a value between 0.0 and 1.0
        let value = 255.0 - (value * 255.0);
        value.round() as u8
    }

    // update
    pub fn update(&mut self, t: u32, display: &Display) {
        let camera = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.01,
        };
        let time_factor = (t as f32 / 1000.0) as f32;
        self.direction.rotate_y(0.1 * time_factor);
        //self.direction.rotate_x(0.1 * time_factor);
        //self.direction.rotate_z(0.1 * time_factor);
        self.direction.normalize();
        let displacement = self.direction.mul(time_factor);
        self.displace(&displacement);
        self.screen_stars.truncate(0); // self.screen_stars.clear();
        for star3d in self.stars.iter() {
            // apply camera displacement
            let cam_star3d = star3d.add(&camera);
            // project to screen space
            let mut point: Point = Point::new();
            point.v = display.project(&cam_star3d);
            let color = self.posterize(-1.0, 1.0, star3d.z);
            point.r = color;
            point.g = color;
            point.b = color;
            self.screen_stars.push(point);
        }
    }

    // render
    pub fn render(&self, display: &mut Display) {
        display.clear_streaming_buffer("starfield", 0, 0, 0);
        let stars_2d = &self.screen_stars;
        let width = display.streaming_buffer_width("starfield").unwrap();
        let height = display.streaming_buffer_height("starfield").unwrap();
        for star in stars_2d.iter() {
            let x: i32 = (star.v.x.round() + (width as f32 / 2.0)) as i32;
            let y: i32 = (star.v.y.round() + (height as f32 / 2.0)) as i32;
            //display.put_pixel(x, y, star.r, star.g, star.b);
            display.put_pixel("starfield", x, y, star.r, star.g, star.b);
        }
        display.streaming_buffer_to_canvas("starfield");
    }
}
