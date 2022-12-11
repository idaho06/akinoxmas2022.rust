use crate::{display::Display, point::Point, scene::{Scene, Sequence}, vector::Vec3};

pub struct Logo {
    order: Vec<Vec3>,
    transformed_3dpoints: Vec<Vec3>,
    screen_points: Vec<Point>,
    camera: Vec3,
    sprites: Vec<(String, String)>,
}

impl Logo {
    pub fn new(display: &mut Display) -> Self {
        // load sprites for logo
        let sprites: Vec<(String, String)> = vec![
            (String::from("logo01"), String::from("assets/logo01.png")),
            (String::from("logo02"), String::from("assets/logo02.png")),
            (String::from("logo03"), String::from("assets/logo03.png")),
            (String::from("logo04"), String::from("assets/logo04.png")),
            (String::from("logo05"), String::from("assets/logo05.png")),
        ];
        for (name, filename) in &sprites {
            display.add_sprite(name.as_str(), filename.as_str());
        }
        Self {
            order: vec![
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 4.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 3.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 2.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            ],
            camera: Vec3 {
                x: 0.5,
                y: 0.5,
                z: 5.0,
            },
            transformed_3dpoints: Vec::<Vec3>::new(),
            screen_points: Vec::<Point>::new(),
            sprites,
        }
    }
}

impl Scene for Logo {
    fn update(&mut self, t: u32, display: &Display, _scene: &Option<Sequence>) {
        let time_factor = (t as f32 / 1000.0) as f32;
        self.camera.rotate_z(1.0 * time_factor);
        /* self.rotation.x += 0.5 * time_factor;
        self.rotation.y += 0.5 * time_factor;
        self.rotation.z += 0.5 * time_factor; */

        // clean Vec of transformed 3D points
        self.transformed_3dpoints.clear();
        // TODO: select tetra, octa or cube
        for point in self.order.iter() {
            // apply rotation
            //let rotated_point = *point;
            /* rotated_point.rotate_x(self.rotation.x);
            rotated_point.rotate_y(self.rotation.y);
            rotated_point.rotate_z(self.rotation.z); */

            self.transformed_3dpoints.push(point.add(&self.camera));
        }
        //self.transformed_3dpoints.sort_unstable_by(|l, r| r.z.total_cmp(&l.z)); // order 3dpoints by z after trasformation

        self.screen_points.truncate(0); //self.screen_points.clean();

        for transformed_3dpoint in self.transformed_3dpoints.iter() {
            // "in self.cube" returns Vec3. "in self.cube.iter()" returns &Vec3
            // project to screen space
            let mut point: Point = Point::new();
            point.v = display.project(transformed_3dpoint);
            point.z = transformed_3dpoint.z;
            point.r = 255_u8;
            point.g = 255_u8;
            point.b = 255_u8;
            self.screen_points.push(point);
        }
    }

    fn render(&self, display: &mut Display) {
        let mut sprites_iter = self.sprites.iter();
        for point in self.screen_points.iter() {
            let sprite_name = sprites_iter.next().unwrap();
            let x: i32 = (point.v.x.round() + (display.w_width() as f32 / 2.0_f32)) as i32;
            let y: i32 = (point.v.y.round() + (display.w_height() as f32 / 2.0_f32)) as i32;
            //display.put_pixel(x, y, point.r, point.g, point.b);
            display.put_sprite_centered(sprite_name.0.as_str(), x, y, 1.0_f32, None);
        }
    }
}
