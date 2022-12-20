use crate::{
    display::Display,
    lerp::remap_f32,
    point::Point,
    scene::{Scene, Sequence},
    vector::{Vec2, Vec3},
};

#[derive(Debug)]
pub struct Platonics {
    tetra: Vec<Vec3>,
    octa: Vec<Vec3>,
    cube: Vec<Vec3>,
    dodec: Vec<Vec3>,
    icos: Vec<Vec3>,
    transformed_3dpoints: Vec<Vec3>,
    screen_points: Vec<Point>,
    rotation: Vec3,
    current_scene: Sequence,
    current_platonic: Vec<Vec3>,
    screen_pos: Vec2,
    start_time: u32,
    end_time: u32,
    now_time: u32,
}

// impl Default for Platonics {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl Platonics {
    pub fn new(display: &mut Display) -> Self {
        //display.add_sprite("particle02", "./assets/particle02.png");
        display.add_sprite("bola01", "./assets/bola_roja.png");
        display.add_sprite("bola02", "./assets/bola_azul.png");
        display.add_sprite("bola03", "./assets/bola_verde.png");
        display.add_sprite("bola04", "./assets/bola_morada.png");
        display.add_sprite("bola05", "./assets/bola_amarilla.png");
        //let gr = ((1.0 + 5.0_f64.sqrt())/2.0) as f32;

        // clippy warning: calls to `push` immediately after creation
        /*
        let mut tetra = Vec::<Vec3>::new();
        tetra.push(Vec3{x: 1_f32, y: 1_f32, z: 1_f32});
        tetra.push(Vec3{x: 1_f32, y: -1_f32, z: -1_f32});
        tetra.push(Vec3{x: -1_f32, y: 1_f32, z: -1_f32});
        tetra.push(Vec3{x: -1_f32, y: -1_f32, z: 1_f32});
        */

        let phi: f32 = (1.0_f32 + 5.0_f32.sqrt()) / 2.0_f32;
        let inv_phi: f32 = 1_f32 / phi;

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

        let icos = vec![
            Vec3 {
                x: 0_f32,
                y: 1_f32,
                z: phi,
            },
            Vec3 {
                x: 0_f32,
                y: 1_f32,
                z: -phi,
            },
            Vec3 {
                x: 0_f32,
                y: -1_f32,
                z: phi,
            },
            Vec3 {
                x: 0_f32,
                y: -1_f32,
                z: -phi,
            },
            Vec3 {
                x: 1_f32,
                y: phi,
                z: 0_f32,
            },
            Vec3 {
                x: 1_f32,
                y: -phi,
                z: 0_f32,
            },
            Vec3 {
                x: -1_f32,
                y: phi,
                z: 0_f32,
            },
            Vec3 {
                x: -1_f32,
                y: -phi,
                z: 0_f32,
            },
            Vec3 {
                x: phi,
                y: 0_f32,
                z: 1_f32,
            },
            Vec3 {
                x: phi,
                y: 0_f32,
                z: -1_f32,
            },
            Vec3 {
                x: -phi,
                y: 0_f32,
                z: 1_f32,
            },
            Vec3 {
                x: -phi,
                y: 0_f32,
                z: -1_f32,
            },
        ];

        let dodec = vec![
            Vec3 {
                x: 1_f32,
                y: 1_f32,
                z: 1_f32,
            },
            Vec3 {
                x: 1_f32,
                y: 1_f32,
                z: -1_f32,
            },
            Vec3 {
                x: 1_f32,
                y: -1_f32,
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
                z: 1_f32,
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
            Vec3 {
                x: -1_f32,
                y: -1_f32,
                z: -1_f32,
            },
            Vec3 {
                x: 0_f32,
                y: inv_phi,
                z: phi,
            },
            Vec3 {
                x: 0_f32,
                y: inv_phi,
                z: -phi,
            },
            Vec3 {
                x: 0_f32,
                y: -inv_phi,
                z: phi,
            },
            Vec3 {
                x: 0_f32,
                y: -inv_phi,
                z: -phi,
            },
            Vec3 {
                x: inv_phi,
                y: phi,
                z: 0_f32,
            },
            Vec3 {
                x: inv_phi,
                y: -phi,
                z: 0_f32,
            },
            Vec3 {
                x: -inv_phi,
                y: phi,
                z: 0_f32,
            },
            Vec3 {
                x: -inv_phi,
                y: -phi,
                z: 0_f32,
            },
            Vec3 {
                x: phi,
                y: 0_f32,
                z: inv_phi,
            },
            Vec3 {
                x: phi,
                y: 0_f32,
                z: -inv_phi,
            },
            Vec3 {
                x: -phi,
                y: 0_f32,
                z: inv_phi,
            },
            Vec3 {
                x: -phi,
                y: 0_f32,
                z: -inv_phi,
            },
        ];

        let current_platonic = vec![];
        Self {
            tetra,
            octa,
            cube,
            dodec,
            icos,
            transformed_3dpoints: Vec::<Vec3>::new(),
            screen_points: Vec::<Point>::new(),
            rotation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            current_scene: Sequence::LogoFallingIn,
            current_platonic,
            screen_pos: Vec2 { x: 0_f32, y: 0_f32 },
            start_time: 0_u32,
            end_time: 0_u32,
            now_time: 0_u32,
        }
    }

    fn reset_to_tetra_in(&mut self, now: u32) {
        //self.screen_pos = Vec2 {
        //    x: 1920_f32 / 2_f32,
        //    y: 0_f32,
        //};
        self.start_time = now;
        self.end_time = now + 3000_u32;
        self.now_time = self.start_time;
        self.current_platonic = self.tetra.clone();
    }
    fn reset_to_tetra_out(&mut self, now: u32) {
        self.start_time = now;
        self.end_time = now + 3000_u32;
        self.now_time = self.start_time;
    }
    fn reset_to_octa_in(&mut self, now: u32) {
        self.start_time = now;
        self.end_time = now + 3000_u32;
        self.now_time = self.start_time;
        self.current_platonic = self.octa.clone();
    }
    fn reset_to_octa_out(&mut self, now: u32) {
        self.start_time = now;
        self.end_time = now + 3000_u32;
        self.now_time = self.start_time;
    }
    fn reset_to_cube_in(&mut self, now: u32) {
        self.start_time = now;
        self.end_time = now + 3000_u32;
        self.now_time = self.start_time;
        self.current_platonic = self.cube.clone();
    }
    fn reset_to_cube_out(&mut self, now: u32) {
        self.start_time = now;
        self.end_time = now + 3000_u32;
        self.now_time = self.start_time;
    }
    fn reset_to_icos_in(&mut self, now: u32) {
        self.start_time = now;
        self.end_time = now + 3000_u32;
        self.now_time = self.start_time;
        self.current_platonic = self.icos.clone();
    }
    fn reset_to_icos_out(&mut self, now: u32) {
        self.start_time = now;
        self.end_time = now + 3000_u32;
        self.now_time = self.start_time;
    }
    fn reset_to_dodec_in(&mut self, now: u32) {
        self.start_time = now;
        self.end_time = now + 3000_u32;
        self.now_time = self.start_time;
        self.current_platonic = self.dodec.clone();
    }
    fn reset_to_dodec_out(&mut self, now: u32) {
        self.start_time = now;
        self.end_time = now + 3000_u32;
        self.now_time = self.start_time;
    }
}

impl Scene for Platonics {
    fn update(&mut self, t: u32, display: &Display, scene: &Option<Sequence>) {
        // check scene change and run updates for the new scene
        if let Some(new_scene) = scene {
            self.current_scene = *new_scene;
            match self.current_scene {
                Sequence::PlatonicsTetraIn => self.reset_to_tetra_in(display.ticks()),
                Sequence::PlatonicsTetraOut => self.reset_to_tetra_out(display.ticks()),
                Sequence::PlatonicsOctaIn => self.reset_to_octa_in(display.ticks()),
                Sequence::PlatonicsOctaOut => self.reset_to_octa_out(display.ticks()),
                Sequence::PlatonicsCubeIn => self.reset_to_cube_in(display.ticks()),
                Sequence::PlatonicsCubeOut => self.reset_to_cube_out(display.ticks()),
                Sequence::PlatonicsIcosIn => self.reset_to_icos_in(display.ticks()),
                Sequence::PlatonicsIcosOut => self.reset_to_icos_out(display.ticks()),
                Sequence::PlatonicsDodecIn => self.reset_to_dodec_in(display.ticks()),
                Sequence::PlatonicsDodecOut => self.reset_to_dodec_out(display.ticks()),
                _ => (), // New scene is not relevant here
            }
        } else {
            self.now_time += t; // no new scene
        }

        // updates specific to the scene
        match self.current_scene {
            Sequence::PlatonicsTetraIn => {
                self.screen_pos.x = remap_f32(
                    self.start_time as f32,
                    self.end_time as f32,
                    1300_f32,
                    0_f32,
                    self.now_time as f32,
                )
                .clamp(0_f32, 1300_f32)
            }
            Sequence::PlatonicsTetraOut => {
                self.screen_pos.x = remap_f32(
                    self.start_time as f32,
                    self.end_time as f32,
                    0_f32,
                    -1500_f32,
                    self.now_time as f32,
                )
                .clamp(-1500_f32, 0_f32)
            }
            Sequence::PlatonicsOctaIn => {
                self.screen_pos.x = remap_f32(
                    self.start_time as f32,
                    self.end_time as f32,
                    1300_f32,
                    0_f32,
                    self.now_time as f32,
                )
                .clamp(0_f32, 1300_f32)
            }
            Sequence::PlatonicsOctaOut => {
                self.screen_pos.x = remap_f32(
                    self.start_time as f32,
                    self.end_time as f32,
                    0_f32,
                    -1500_f32,
                    self.now_time as f32,
                )
                .clamp(-1500_f32, 0_f32)
            }
            Sequence::PlatonicsCubeIn => {
                self.screen_pos.x = remap_f32(
                    self.start_time as f32,
                    self.end_time as f32,
                    1300_f32,
                    0_f32,
                    self.now_time as f32,
                )
                .clamp(0_f32, 1300_f32)
            }
            Sequence::PlatonicsCubeOut => {
                self.screen_pos.x = remap_f32(
                    self.start_time as f32,
                    self.end_time as f32,
                    0_f32,
                    -1500_f32,
                    self.now_time as f32,
                )
                .clamp(-1500_f32, 0_f32)
            }
            Sequence::PlatonicsIcosIn => {
                self.screen_pos.x = remap_f32(
                    self.start_time as f32,
                    self.end_time as f32,
                    1300_f32,
                    0_f32,
                    self.now_time as f32,
                )
                .clamp(0_f32, 1300_f32)
            }
            Sequence::PlatonicsIcosOut => {
                self.screen_pos.x = remap_f32(
                    self.start_time as f32,
                    self.end_time as f32,
                    0_f32,
                    -1500_f32,
                    self.now_time as f32,
                )
                .clamp(-1500_f32, 0_f32)
            }
            Sequence::PlatonicsDodecIn => {
                self.screen_pos.x = remap_f32(
                    self.start_time as f32,
                    self.end_time as f32,
                    1300_f32,
                    0_f32,
                    self.now_time as f32,
                )
                .clamp(0_f32, 1300_f32)
            }
            Sequence::PlatonicsDodecOut => {
                self.screen_pos.x = remap_f32(
                    self.start_time as f32,
                    self.end_time as f32,
                    0_f32,
                    -1500_f32,
                    self.now_time as f32,
                )
                .clamp(-1500_f32, 0_f32)
            }
            _ => return,
        }

        // updates common to all scenes
        let camera = Vec3 {
            x: 0.0_f32,
            y: 0.0_f32,
            z: 4.0_f32,
        };
        let time_factor = (t as f32 / 1000.0) as f32;
        self.rotation.x += 0.5 * time_factor;
        self.rotation.y += 0.5 * time_factor;
        self.rotation.z += 1.0 * time_factor;

        // clean Vec of transformed 3D points
        self.transformed_3dpoints.clear();
        // Select tetra, octa, cube, icos or dodec with self.current_platonic
        for point in self.current_platonic.iter() {
            // apply rotation
            let mut rotated_point = *point;
            rotated_point.rotate_x(self.rotation.x);
            rotated_point.rotate_y(self.rotation.y);
            rotated_point.rotate_z(self.rotation.z);

            self.transformed_3dpoints.push(rotated_point.add(&camera));
        }
        self.transformed_3dpoints
            .sort_unstable_by(|l, r| r.z.total_cmp(&l.z)); // order 3dpoints by z after trasformation

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
        // clippy warning: unneeded late initialization
        //let name: &str;
        //match self.current_scene {
        //    Sequence::PlatonicsTetraIn => name = "bola01",
        //    Sequence::PlatonicsTetraOut => name = "bola01",
        //    _ => return,
        //}
        let name: &str = match self.current_scene {
            Sequence::PlatonicsTetraIn => "bola01",
            Sequence::PlatonicsTetraOut => "bola01",
            Sequence::PlatonicsOctaIn => "bola02",
            Sequence::PlatonicsOctaOut => "bola02",
            Sequence::PlatonicsCubeIn => "bola03",
            Sequence::PlatonicsCubeOut => "bola03",
            Sequence::PlatonicsIcosIn => "bola04",
            Sequence::PlatonicsIcosOut => "bola04",
            Sequence::PlatonicsDodecIn => "bola05",
            Sequence::PlatonicsDodecOut => "bola05",
            _ => return,
        };

        for point in self.screen_points.iter() {
            let x: i32 =
                (point.v.x + (display.w_width() as f32 / 2.0_f32) + self.screen_pos.x) as i32;
            let y: i32 =
                (point.v.y + (display.w_height() as f32 / 2.0_f32) + self.screen_pos.y) as i32;
            //display.put_pixel(x, y, point.r, point.g, point.b);
            let size_factor = 3_f32 / point.z;
            let light_factor = size_factor.clamp(0.0_f32, 1.0_f32);
            let color = 255.0_f32 * light_factor;
            let mod_color = Some((color as u8, color as u8, color as u8));
            display.put_sprite_centered(name, x, y, size_factor, mod_color);
            // change size_factor to Option type
        }
    }
}
