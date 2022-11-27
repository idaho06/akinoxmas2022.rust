use crate::{point::Point, vector::Vec3, display::Display};

#[derive(Debug)]
pub struct Platonics {
    tetra: Vec<Vec3>,
    octa: Vec<Vec3>,
    cube: Vec<Vec3>,
    //dodec: Vec<Vec3>,
    //icos: Vec<Vec3>,
    transformed_3dpoints: Vec::<Vec3>,
    screen_points: Vec<Point>,
    rotation: Vec3,
}

// impl Default for Platonics {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl Platonics {
    pub fn new(display: &mut Display) -> Self {
        display.add_sprite("particle02", "./assets/particle02.png");
        //let gr = ((1.0 + 5.0_f64.sqrt())/2.0) as f32;

        // clippy warning: calls to `push` immediately after creation
        /*
        let mut tetra = Vec::<Vec3>::new();
        tetra.push(Vec3{x: 1_f32, y: 1_f32, z: 1_f32});
        tetra.push(Vec3{x: 1_f32, y: -1_f32, z: -1_f32});
        tetra.push(Vec3{x: -1_f32, y: 1_f32, z: -1_f32});
        tetra.push(Vec3{x: -1_f32, y: -1_f32, z: 1_f32});
        */
        let tetra = vec![
            Vec3 {
                x: 1_f32,
                y: 1_f32,
                z: 1_f32,
            },
            Vec3 {
                x: 1_f32,
                y: -1_f32,
                z: -1_f32,
            },
            Vec3 {
                x: -1_f32,
                y: 1_f32,
                z: -1_f32,
            },
            Vec3 {
                x: -1_f32,
                y: -1_f32,
                z: 1_f32,
            },
        ];

        let octa = vec![
            Vec3 {
                x: 1_f32,
                y: 0_f32,
                z: 0_f32,
            },
            Vec3 {
                x: -1_f32,
                y: 0_f32,
                z: 0_f32,
            },
            Vec3 {
                x: 0_f32,
                y: 1_f32,
                z: 0_f32,
            },
            Vec3 {
                x: 0_f32,
                y: -1_f32,
                z: 0_f32,
            },
            Vec3 {
                x: 0_f32,
                y: 0_f32,
                z: 1_f32,
            },
            Vec3 {
                x: 0_f32,
                y: 0_f32,
                z: -1_f32,
            },
        ];

        let cube = vec![
            Vec3 {
                x: -1_f32,
                y: -1_f32,
                z: -1_f32,
            },
            Vec3 {
                x: -1_f32,
                y: 1_f32,
                z: -1_f32,
            },
            Vec3 {
                x: 1_f32,
                y: 1_f32,
                z: -1_f32,
            },
            Vec3 {
                x: 1_f32,
                y: -1_f32,
                z: -1_f32,
            },
            Vec3 {
                x: 1_f32,
                y: 1_f32,
                z: 1_f32,
            },
            Vec3 {
                x: 1_f32,
                y: -1_f32,
                z: 1_f32,
            },
            Vec3 {
                x: -1_f32,
                y: 1_f32,
                z: 1_f32,
            },
            Vec3 {
                x: -1_f32,
                y: -1_f32,
                z: 1_f32,
            },
        ];

        Self {
            tetra,
            octa,
            cube,
            //dodec,
            //icos,
            transformed_3dpoints: Vec::<Vec3>::new(),
            screen_points: Vec::<Point>::new(),
            rotation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }

    pub fn update(&mut self, t: u32, display: &Display) {
        let camera = Vec3 {
            x: 0.0_f32,
            y: 0.0_f32,
            z: 5.0_f32,
        };
        let time_factor = (t as f32 / 1000.0) as f32;
        self.rotation.x += 0.5 * time_factor;
        self.rotation.y += 0.5 * time_factor;
        self.rotation.z += 0.5 * time_factor;

        // clean Vec of transformed 3D points
        self.transformed_3dpoints.clear();
        // TODO: select tetra, octa or cube
        for point in self.cube.iter() {
            // apply rotation
            let mut rotated_point = *point;
            rotated_point.rotate_x(self.rotation.x);
            rotated_point.rotate_y(self.rotation.y);
            rotated_point.rotate_z(self.rotation.z);

            self.transformed_3dpoints.push(rotated_point.add(&camera));
        }
        self.transformed_3dpoints.sort_unstable_by(|l, r| l.z.total_cmp(&r.z)); // order 3dpoints by z after trasformation

        self.screen_points.truncate(0); //self.screen_points.clean();
        
        for transformed_3dpoint in self.transformed_3dpoints.iter() {
            // "in self.cube" returns Vec3. "in self.cube.iter()" returns &Vec3
            // project to screen space
            let mut point: Point = Point::new();
            point.v = display.project(transformed_3dpoint);
            point.r = 255_u8;
            point.g = 255_u8;
            point.b = 255_u8;
            self.screen_points.push(point);
        }
        
    }

    pub fn render(&self, display: &mut Display) {
        for point in self.screen_points.iter() {
            let x: i32 = (point.v.x.round() + (display.w_width() as f32 / 2.0_f32)) as i32; // TODO: change this to w_width and w_height
            let y: i32 = (point.v.y.round() + (display.w_height() as f32 / 2.0_f32)) as i32;
            //display.put_pixel(x, y, point.r, point.g, point.b);
            display.put_sprite("particle02", x, y, 1.0_f32);
        }
    }
}
