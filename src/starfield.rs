use crate::vector::{Vec3, Vec2};
use rand::Rng;


pub struct Starfield {
    stars: Vec<Vec3>,
    limits: (f32, f32),
    screen_stars: Vec<Vec2>,
}

impl Starfield {
    pub fn new() -> Self {
        // Create a vector of random Vec3 in space between (-1.0, -1.0, -1,0) to (1.0, 1.0, 1.0)
        let num_stars: usize = 2000;
        let zero_vec = Vec3{x: 0.0, y: 0.0, z: 0.0};
        let mut stars = vec![zero_vec; num_stars];
        let mut rng = rand::thread_rng();
        let limits = (-1.0, 1.0);

        for star in stars.iter_mut(){
            star.x = rng.gen_range(limits.0..limits.1);
            star.y = rng.gen_range(limits.0..limits.1);
            star.z = rng.gen_range(limits.0..limits.1);
        }

        //println!("{:?}", stars);

        Self { 
            stars: stars, 
            limits: limits,
            screen_stars: Vec::<Vec2>::new(),
        }

    }

    pub fn stars(&self) -> &Vec<Vec3> {
        &self.stars
    }

    pub fn displace(&mut self, v: &Vec3){
        for star in self.stars.iter_mut(){
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

    // update

    // render
}