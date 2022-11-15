/* 
use display::Display;
use point::Point;
use starfield::Starfield;
use vector::{Vec2, Vec3};
*/

pub mod display;
pub mod vector;
pub mod starfield;
pub mod point;

/* 
fn posterize(min: f32, max: f32, value: f32) -> u8 {
    if value < min { return 0xff }
    if value > max { return 0x00 }
    let delta = max - min;
    let value = value - min;
    let value = value / delta; // this should return a value between 0.0 and 1.0
    let value = 255.0 - (value * 255.0);
    value.round() as u8
}

fn project(v: &Vec3) -> Vec2 {
    let fov_factor: f32 = 320.0;
    Vec2 { 
        x: (fov_factor * v.x) / v.z , 
        y: (fov_factor * -v.y) / v.z ,
    }
}

pub fn update_starfield(t: u32, dir: &Vec3, stf3d: &mut Starfield, stf2d: &mut Vec<Point>) {
    let camera = Vec3{x:0.0, y:0.0, z:1.01};
    let time_factor = (t as f32 / 1000.0) as f32;
    let displacement = dir.mul(time_factor);
    stf3d.displace(&displacement);
    stf2d.truncate(0); // stf2d.clear();
    for star3d in stf3d.stars().iter() {
        // apply camera displacement
        let cam_star3d = star3d.add(&camera);
        // project to screen space
        let mut point: Point = Point::new();
        point.v = project(&cam_star3d);
        let color = posterize(-1.0, 1.0, star3d.z);
        point.r = color;
        point.g = color;
        point.b = color;
        stf2d.push(point);
    }

}

pub fn render_starfield(display: &mut Display, stars_2d: &Vec<Point>) {
    for star in stars_2d.iter() {
        let x: i32 = (star.v.x.round() + (display.t_width() as f32 / 2.0)) as i32;
        let y: i32 = (star.v.y.round() + (display.t_height() as f32 / 2.0)) as i32;
        display.put_pixel(x, y, star.r, star.g, star.b);
    }
}
*/