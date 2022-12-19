use std::f32::consts::{PI, TAU};

use crate::{
    display::Display,
    lerp::remap_f32,
    point::{Pixel, Point},
    scene::{Scene, Sequence},
    triangle::Triangle,
    vector::{Vec2, Vec3},
};

//const VS_SEGMENT: usize = 10; // vertices per segment
//const SEGMENTS: usize = 14; // number of segments
//const S_RADIUS: f32 = 0.25; // radius of segment
//const T_RADIUS: f32 = 0.75; // center of torus to center of segment radius

struct TorusFace {
    t1: Triangle,
    t2: Triangle,
    z: f32,
    color: usize,
}

pub struct Torus {
    vertices: Vec<Vec3>,             // static list of vertices. DO NOT SORT
    transformed_3dpoints: Vec<Vec3>, // vertices transformed in 3D space. DO NOT SORT
    faces: Vec<TorusFace>, // faces of the torus. triangles pointing to the vertices indexes in transformed_3dpoints/vertices. SORT FOR PAINTER ALGORITHM
    screen_points: Vec<Point>, // vertices projected to screen space, not centered. DO NOT SORT
    pixel_queue: Vec<Pixel>, // List of pixels to draw
    rotation: Vec3,
    colors: Vec<u32>,
    current_scene: Sequence,
    camera: Vec3,
    //screen_pos: Vec2,
    start_time: u32,
    end_time: u32,
    now_time: u32,
    faces_in_screen: f32,
    color_rotate_milis: u32,
    color_rotate_limit: u32,
}

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

impl Torus {
    pub fn new(display: &mut Display) -> Self {
        // colors
        let colors = vec![];
        // create stream_buffer in display
        display.add_streaming_buffer("torus", WIDTH, HEIGHT);
        let vertices = vec![];
        let faces = vec![];

        Self {
            vertices,
            faces,
            transformed_3dpoints: Vec::new(),
            screen_points: Vec::new(),
            pixel_queue: Vec::<Pixel>::new(),
            rotation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            colors,
            current_scene: Sequence::LogoFallingIn,
            camera: Vec3 {
                x: 0.0_f32,
                y: 0.0_f32,
                z: 5.0_f32,
            },
            //screen_pos: Vec2 { x: 0_f32, y: 0_f32 },
            start_time: 0_u32,
            end_time: 0_u32,
            now_time: 0_u32,
            faces_in_screen: 0_f32,
            color_rotate_limit: 0_u32,
            color_rotate_milis: 0_u32,
        }
    }

    fn build_torus(
        vs_segment: usize,
        segments: usize,
        s_radius: f32,
        t_radius: f32,
        num_colors: usize,
    ) -> (Vec<Vec3>, Vec<TorusFace>) {
        let mut vertices_torus = Vec::<Vec3>::new();
        let mut vertices_segment = Vec::<Vec3>::new();

        let angle_in_segment = TAU / vs_segment as f32; // TAU = PI*2
        let angle_between_segments = TAU / segments as f32;
        let segment_radius = Vec3 {
            x: 0.0,
            y: s_radius,
            z: 0.0,
        };
        let torus_radius = Vec3 {
            x: t_radius,
            y: 0.0,
            z: 0.0,
        };

        // create new vertex at origin
        let v = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        // translate in y the segment radius
        let mut v_segment = v.add(&segment_radius);
        for _ in 0..vs_segment {
            vertices_segment.push(v_segment);
            // rotate vertex
            v_segment = *vertices_segment.last().unwrap();
            v_segment.rotate_z(angle_in_segment);
        }
        // translate segment to torus
        vertices_segment
            .iter_mut()
            .for_each(|v| *v = v.add(&torus_radius));
        // clone and rotate by number of segments - 1
        for segment in 0..segments {
            // copy
            let mut clone_segment = vertices_segment.clone();
            //rotate
            clone_segment
                .iter_mut()
                .for_each(|v| v.rotate_y(angle_between_segments * segment as f32));
            //append to list of vertices
            vertices_torus.append(&mut clone_segment);
        }
        //let size = vertices_torus.len();

        // calculate torus faces on the vertices
        let mut faces = Vec::<TorusFace>::new();

        let torus_width = segments;
        let torus_height = vs_segment;
        let torus_size = segments * vs_segment;
        let mut v1_index: usize;
        let mut v2_index: usize;
        let mut v3_index: usize;
        let mut v4_index: usize;

        let mut color = 0_usize;
        for i in 0..torus_width {
            for j in 0..torus_height - 1 {
                v1_index = i * torus_height + j;
                v2_index = v1_index + 1;
                v3_index = v1_index + torus_height;
                v4_index = v3_index + 1;

                let mut t1 = Triangle::new();
                t1.v1 = v1_index % torus_size;
                t1.v2 = v2_index % torus_size;
                t1.v3 = v3_index % torus_size;
                t1.r = 255_u8;
                let mut t2 = Triangle::new();
                t2.v1 = v3_index % torus_size;
                t2.v2 = v2_index % torus_size;
                t2.v3 = v4_index % torus_size;
                t2.g = 255_u8;

                let face = TorusFace {
                    t1,
                    t2,
                    z: 0_f32,
                    color: color % num_colors,
                };
                faces.push(face);
                color += 1;
            }
            v1_index = (i * torus_height) + torus_height - 1;
            v2_index = i * torus_height;
            v3_index = v1_index + torus_height;
            v4_index = v1_index + 1;
            let mut t1 = Triangle::new();
            t1.v1 = v1_index % torus_size;
            t1.v2 = v2_index % torus_size;
            t1.v3 = v3_index % torus_size;
            t1.r = 255_u8;
            let mut t2 = Triangle::new();
            t2.v1 = v3_index % torus_size;
            t2.v2 = v2_index % torus_size;
            t2.v3 = v4_index % torus_size;
            t2.g = 255_u8;
            let face = TorusFace {
                t1,
                t2,
                z: 0_f32,
                color: color % num_colors,
            };
            faces.push(face);
            color += 1;
        }

        (vertices_torus, faces)
    }

    fn get_pallete_ega() -> Vec<u32> {
        vec![
            0xFF000000_u32,
            0xFF0000AA_u32,
            0xFF00AA00_u32,
            0xFF00AAAA_u32,
            0xFFAA0000_u32,
            0xFFAA00AA_u32,
            0xFFAAAA00_u32,
            0xFFAAAAAA_u32,
            0xFF000055_u32,
            0xFF0000FF_u32,
            0xFF00AA55_u32,
            0xFF00AAFF_u32,
            0xFFAA0055_u32,
            0xFFAA00FF_u32,
            0xFFAAAA55_u32,
            0xFFAAAAFF_u32,
            0xFF005500_u32,
            0xFF0055AA_u32,
            0xFF00FF00_u32,
            0xFF00FFAA_u32,
            0xFFAA5500_u32,
            0xFFAA55AA_u32,
            0xFFAAFF00_u32,
            0xFFAAFFAA_u32,
            0xFF005555_u32,
            0xFF0055FF_u32,
            0xFF00FF55_u32,
            0xFF00FFFF_u32,
            0xFFAA5555_u32,
            0xFFAA55FF_u32,
            0xFFAAFF55_u32,
            0xFFAAFFFF_u32,
            0xFF550000_u32,
            0xFF5500AA_u32,
            0xFF55AA00_u32,
            0xFF55AAAA_u32,
            0xFFFF0000_u32,
            0xFFFF00AA_u32,
            0xFFFFAA00_u32,
            0xFFFFAAAA_u32,
            0xFF550055_u32,
            0xFF5500FF_u32,
            0xFF55AA55_u32,
            0xFF55AAFF_u32,
            0xFFFF0055_u32,
            0xFFFF00FF_u32,
            0xFFFFAA55_u32,
            0xFFFFAAFF_u32,
            0xFF555500_u32,
            0xFF5555AA_u32,
            0xFF55FF00_u32,
            0xFF55FFAA_u32,
            0xFFFF5500_u32,
            0xFFFF55AA_u32,
            0xFFFFFF00_u32,
            0xFFFFFFAA_u32,
            0xFF555555_u32,
            0xFF5555FF_u32,
            0xFF55FF55_u32,
            0xFF55FFFF_u32,
            0xFFFF5555_u32,
            0xFFFF55FF_u32,
            0xFFFFFF55_u32,
            0xFFFFFFFF_u32,
        ] //ARGB888
    }

    fn get_pallete_vga() -> Vec<u32> {
        vec![
            0xFF000000, 0xFF0000AA, 0xFF00AA00, 0xFF00AAAA, 0xFFAA0000, 0xFFAA00AA, 0xFFAA5500,
            0xFFAAAAAA, 0xFF555555, 0xFF5555FF, 0xFF55FF55, 0xFF55FFFF, 0xFFFF5555, 0xFFFF55FF,
            0xFFFFFF55, 0xFFFFFFFF, 0xFF000000, 0xFF101010, 0xFF202020, 0xFF353535, 0xFF454545,
            0xFF555555, 0xFF656565, 0xFF757575, 0xFF8A8A8A, 0xFF9A9A9A, 0xFFAAAAAA, 0xFFBABABA,
            0xFFCACACA, 0xFFDFDFDF, 0xFFEFEFEF, 0xFFFFFFFF, 0xFF0000FF, 0xFF4100FF, 0xFF8200FF,
            0xFFBE00FF, 0xFFFF00FF, 0xFFFF00BE, 0xFFFF0082, 0xFFFF0041, 0xFFFF0000, 0xFFFF4100,
            0xFFFF8200, 0xFFFFBE00, 0xFFFFFF00, 0xFFBEFF00, 0xFF82FF00, 0xFF41FF00, 0xFF00FF00,
            0xFF00FF41, 0xFF00FF82, 0xFF00FFBE, 0xFF00FFFF, 0xFF00BEFF, 0xFF0082FF, 0xFF0041FF,
            0xFF8282FF, 0xFF9E82FF, 0xFFBE82FF, 0xFFDF82FF, 0xFFFF82FF, 0xFFFF82DF, 0xFFFF82BE,
            0xFFFF829E, 0xFFFF8282, 0xFFFF9E82, 0xFFFFBE82, 0xFFFFDF82, 0xFFFFFF82, 0xFFDFFF82,
            0xFFBEFF82, 0xFF9EFF82, 0xFF82FF82, 0xFF82FF9E, 0xFF82FFBE, 0xFF82FFDF, 0xFF82FFFF,
            0xFF82DFFF, 0xFF82BEFF, 0xFF829EFF, 0xFFBABAFF, 0xFFCABAFF, 0xFFDFBAFF, 0xFFEFBAFF,
            0xFFFFBAFF, 0xFFFFBAEF, 0xFFFFBADF, 0xFFFFBACA, 0xFFFFBABA, 0xFFFFCABA, 0xFFFFDFBA,
            0xFFFFEFBA, 0xFFFFFFBA, 0xFFEFFFBA, 0xFFDFFFBA, 0xFFCAFFBA, 0xFFBAFFBA, 0xFFBAFFCA,
            0xFFBAFFDF, 0xFFBAFFEF, 0xFFBAFFFF, 0xFFBAEFFF, 0xFFBADFFF, 0xFFBACAFF, 0xFF000071,
            0xFF1C0071, 0xFF390071, 0xFF550071, 0xFF710071, 0xFF710055, 0xFF710039, 0xFF71001C,
            0xFF710000, 0xFF711C00, 0xFF713900, 0xFF715500, 0xFF717100, 0xFF557100, 0xFF397100,
            0xFF1C7100, 0xFF007100, 0xFF00711C, 0xFF007139, 0xFF007155, 0xFF007171, 0xFF005571,
            0xFF003971, 0xFF001C71, 0xFF393971, 0xFF453971, 0xFF553971, 0xFF613971, 0xFF713971,
            0xFF713961, 0xFF713955, 0xFF713945, 0xFF713939, 0xFF714539, 0xFF715539, 0xFF716139,
            0xFF717139, 0xFF617139, 0xFF557139, 0xFF457139, 0xFF397139, 0xFF397145, 0xFF397155,
            0xFF397161, 0xFF397171, 0xFF396171, 0xFF395571, 0xFF394571, 0xFF515171, 0xFF595171,
            0xFF615171, 0xFF695171, 0xFF715171, 0xFF715169, 0xFF715161, 0xFF715159, 0xFF715151,
            0xFF715951, 0xFF716151, 0xFF716951, 0xFF717151, 0xFF697151, 0xFF617151, 0xFF597151,
            0xFF517151, 0xFF517159, 0xFF517161, 0xFF517169, 0xFF517171, 0xFF516971, 0xFF516171,
            0xFF515971, 0xFF000041, 0xFF100041, 0xFF200041, 0xFF310041, 0xFF410041, 0xFF410031,
            0xFF410020, 0xFF410010, 0xFF410000, 0xFF411000, 0xFF412000, 0xFF413100, 0xFF414100,
            0xFF314100, 0xFF204100, 0xFF104100, 0xFF004100, 0xFF004110, 0xFF004120, 0xFF004131,
            0xFF004141, 0xFF003141, 0xFF002041, 0xFF001041, 0xFF202041, 0xFF282041, 0xFF312041,
            0xFF392041, 0xFF412041, 0xFF412039, 0xFF412031, 0xFF412028, 0xFF412020, 0xFF412820,
            0xFF413120, 0xFF413920, 0xFF414120, 0xFF394120, 0xFF314120, 0xFF284120, 0xFF204120,
            0xFF204128, 0xFF204131, 0xFF204139, 0xFF204141, 0xFF203941, 0xFF203141, 0xFF202841,
            0xFF2D2D41, 0xFF312D41, 0xFF352D41, 0xFF3D2D41, 0xFF412D41, 0xFF412D3D, 0xFF412D35,
            0xFF412D31, 0xFF412D2D, 0xFF41312D, 0xFF41352D, 0xFF413D2D, 0xFF41412D, 0xFF3D412D,
            0xFF35412D, 0xFF31412D, 0xFF2D412D, 0xFF2D4131, 0xFF2D4135, 0xFF2D413D, 0xFF2D4141,
            0xFF2D3D41, 0xFF2D3541, 0xFF2D3141, 0xFF000000, 0xFF000000, 0xFF000000, 0xFF000000,
            0xFF000000, 0xFF000000, 0xFF000000, 0xFF000000,
        ]
    }
    fn get_palette_cga1() -> Vec<u32> {
        vec![0xFF000000, 0xff55ffff, 0xffff55ff, 0xffffffff]
    }
    // fn get_palette_cga2() -> Vec<u32> {
    //     vec![0xFF000000, 0xff55ff55, 0xffff5555, 0xffffff55]
    // }

    fn reset_to_torus01_in(&mut self, now: u32) {
        self.faces_in_screen = 0_f32;
        self.camera = Vec3 {
            x: 0.0_f32,
            y: 0.0_f32,
            z: 5.0_f32,
        };
        self.rotation = Vec3 {
            x: -0.4,
            y: 0.0,
            z: 0.2,
        };
        self.start_time = now;
        self.end_time = now + 3000_u32;
        self.now_time = self.start_time;
        self.colors = Torus::get_pallete_ega();
        (self.vertices, self.faces) = Torus::build_torus(10, 14, 0.25, 0.75, 16);
    }
    fn reset_to_torus01_color_rotate(&mut self, now: u32) {
        self.start_time = now;
        self.end_time = now + 5000_u32;
        self.now_time = self.start_time;
        self.color_rotate_limit = 150_u32;
        self.color_rotate_milis = 0_u32;
    }
    fn reset_to_torus01_rotate_y(&mut self, now: u32) {
        self.start_time = now;
        self.end_time = now + 10_000_u32;
        self.now_time = self.start_time;
    }
    fn reset_to_torus01_out(&mut self, now: u32) {
        self.start_time = now;
        self.end_time = now + 5_000_u32;
        self.now_time = self.start_time;
    }
    fn reset_to_torus02_in(&mut self, now: u32) {
        self.camera = Vec3 {
            x: 0.0_f32,
            y: 0.0_f32,
            z: 250.0_f32,
        };
        self.rotation = Vec3 {
            x: -PI / 2_f32,
            y: 0.0,
            z: 0.0,
        };
        self.start_time = now;
        self.end_time = now + 3000_u32;
        self.now_time = self.start_time;
        self.colors = Torus::get_pallete_vga();
        (self.vertices, self.faces) = Torus::build_torus(24, 48, 0.25, 0.75, 104);
        self.faces_in_screen = self.faces.len() as f32;
    }
    fn reset_to_torus02_rotate_z(&mut self, now: u32) {
        self.start_time = now;
        self.end_time = now + 3_000_u32;
        self.now_time = self.start_time;
        self.color_rotate_limit = 50_u32;
        self.color_rotate_milis = 0_u32;
    }
    fn reset_to_torus02_out(&mut self, now: u32) {
        self.start_time = now;
        self.end_time = now + 2_000_u32;
        self.now_time = self.start_time;
    }
    fn reset_to_torus03_in(&mut self, now: u32) {
        self.camera = Vec3 {
            x: 0.0_f32,
            y: 0.0_f32,
            z: 5.0_f32,
        };
        self.rotation = Vec3 {
            x: -0.5,
            y: -0.1,
            z: 0.0,
        };
        self.start_time = now;
        self.end_time = now + 2000_u32;
        self.now_time = self.start_time;
        self.colors = Torus::get_palette_cga1();
        (self.vertices, self.faces) = Torus::build_torus(6, 8, 0.1, 0.9, 4);
        self.faces_in_screen = self.faces.len() as f32;
        self.color_rotate_limit = 300_u32;
        self.color_rotate_milis = 0_u32;
    }
    fn reset_to_torus03_crazy_rotate(&mut self, now: u32) {
        self.start_time = now;
        self.end_time = now + 10_000_u32;
        self.now_time = self.start_time;
    }
    fn reset_to_torus04_in(&mut self, now: u32) {
        self.camera = Vec3 {
            x: 0.0_f32,
            y: 0.0_f32,
            z: 4.0_f32,
        };
        self.rotation = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        self.start_time = now;
        self.end_time = now + 4000_u32;
        self.now_time = self.start_time;
        self.colors = Torus::get_pallete_vga();
        (self.vertices, self.faces) = Torus::build_torus(32, 32, 0.5, 0.5, 256);
        self.faces_in_screen = 0_f32;
        self.color_rotate_limit = 300_u32;
        self.color_rotate_milis = 0_u32;
    }
    fn reset_to_torus04_out(&mut self, now: u32) {
        self.start_time = now;
        self.end_time = now + 4_000_u32;
        self.now_time = self.start_time;
    }
}

impl Scene for Torus {
    // implement update
    fn update(&mut self, t: u32, display: &Display, scene: &Option<Sequence>) {
        // clear the pixel queue
        self.pixel_queue.truncate(0);
        // check scene change and run updates for the new scene
        if let Some(new_scene) = scene {
            self.current_scene = *new_scene;
            match self.current_scene {
                Sequence::Torus01In => self.reset_to_torus01_in(display.ticks()),
                Sequence::Torus01ColorRotate => self.reset_to_torus01_color_rotate(display.ticks()),
                Sequence::Torus01RotateY => self.reset_to_torus01_rotate_y(display.ticks()),
                Sequence::Torus01Out => self.reset_to_torus01_out(display.ticks()),
                Sequence::Torus02In => self.reset_to_torus02_in(display.ticks()),
                Sequence::Torus02RotateZ => self.reset_to_torus02_rotate_z(display.ticks()),
                Sequence::Torus02Out => self.reset_to_torus02_out(display.ticks()),
                Sequence::Torus03In => self.reset_to_torus03_in(display.ticks()),
                Sequence::Torus03CrazyRotate => self.reset_to_torus03_crazy_rotate(display.ticks()),
                //Sequence::Torus03Out => self.reset_to_torus03_out(display.ticks()),
                Sequence::Torus04In => self.reset_to_torus04_in(display.ticks()),
                //Sequence::Torus04Rotate => self.reset_to_torus04_rotate(display.ticks()),
                Sequence::Torus04Out => self.reset_to_torus04_out(display.ticks()),
                _ => (), // New scene is not relevant here
            }
        } else {
            self.now_time += t; // no new scene
        }

        let time_factor = (t as f32 / 1000.0) as f32;

        // updates specific to the scene
        match self.current_scene {
            Sequence::Torus01In => {
                self.faces_in_screen = remap_f32(
                    self.start_time as f32,
                    self.end_time as f32,
                    0_f32,
                    self.faces.len() as f32,
                    self.now_time as f32,
                )
                .clamp(0_f32, self.faces.len() as f32)
            }
            Sequence::Torus01ColorRotate => {
                self.color_rotate_milis += t;
                if self.color_rotate_milis > self.color_rotate_limit {
                    self.color_rotate_milis -= self.color_rotate_limit;
                    self.colors.rotate_right(1);
                }
            }
            Sequence::Torus01RotateY => {
                self.color_rotate_milis += t;
                if self.color_rotate_milis > self.color_rotate_limit {
                    self.color_rotate_milis -= self.color_rotate_limit;
                    self.colors.rotate_right(1);
                }
                self.rotation.y += 0.8 * time_factor;
            }
            Sequence::Torus01Out => {
                self.color_rotate_milis += t;
                if self.color_rotate_milis > self.color_rotate_limit {
                    self.color_rotate_milis -= self.color_rotate_limit;
                    self.colors.rotate_right(1);
                }
                self.rotation.y += 0.8 * time_factor;
                self.camera.z = remap_f32(
                    self.start_time as f32,
                    self.end_time as f32,
                    5_f32,
                    500_f32,
                    self.now_time as f32,
                );
            }
            Sequence::Torus02In => {
                self.rotation.z += 1.8 * time_factor;
                self.camera.z = remap_f32(
                    self.start_time as f32,
                    self.end_time as f32,
                    120_f32,
                    1_f32,
                    self.now_time as f32,
                );
            }
            Sequence::Torus02RotateZ => {
                self.color_rotate_milis += t;
                if self.color_rotate_milis > self.color_rotate_limit {
                    self.color_rotate_milis -= self.color_rotate_limit;
                    self.colors.rotate_right(1);
                }
                self.rotation.z -= 1.2 * time_factor;
                self.camera.z = remap_f32(
                    self.start_time as f32,
                    self.end_time as f32,
                    1_f32,
                    5.5_f32,
                    self.now_time as f32,
                )
                .clamp(1_f32, 5.5_f32);
            }
            Sequence::Torus02Out => {
                self.color_rotate_milis += t;
                if self.color_rotate_milis > self.color_rotate_limit {
                    self.color_rotate_milis -= self.color_rotate_limit;
                    self.colors.rotate_right(1);
                }
                self.rotation.z -= 1.2 * time_factor;
                self.camera.x = remap_f32(
                    self.start_time as f32,
                    self.end_time as f32,
                    0_f32,
                    5_f32,
                    self.now_time as f32,
                );
            }
            Sequence::Torus03In => {
                self.color_rotate_milis += t;
                if self.color_rotate_milis > self.color_rotate_limit {
                    self.color_rotate_milis -= self.color_rotate_limit;
                    self.colors.rotate_right(1);
                }
                //self.rotation.z -= 1.2 * time_factor;
                self.camera.x = remap_f32(
                    self.start_time as f32,
                    self.end_time as f32,
                    -5_f32,
                    0_f32,
                    self.now_time as f32,
                )
                .clamp(-5_f32, 0_f32);
            }
            Sequence::Torus03CrazyRotate => {
                self.color_rotate_milis += t;
                if self.color_rotate_milis > self.color_rotate_limit {
                    self.color_rotate_milis -= self.color_rotate_limit;
                    self.colors.rotate_right(1);
                }
                self.rotation.x +=
                    (self.now_time - self.start_time) as f32 * time_factor * time_factor * 0.05;
                self.rotation.z +=
                    (self.now_time - self.start_time) as f32 * time_factor * time_factor * 0.02;
            }
            Sequence::Torus03Out => {
                self.color_rotate_milis += t;
                if self.color_rotate_milis > self.color_rotate_limit {
                    self.color_rotate_milis -= self.color_rotate_limit;
                    self.colors.rotate_right(1);
                }
                self.rotation.x +=
                    (self.now_time - self.start_time) as f32 * time_factor * time_factor * 0.05;
                self.rotation.z +=
                    (self.now_time - self.start_time) as f32 * time_factor * time_factor * 0.02;

                self.camera.x += 1.8 * time_factor;
            }
            Sequence::Torus04In => {
                self.color_rotate_milis += t;
                if self.color_rotate_milis > self.color_rotate_limit {
                    self.color_rotate_milis -= self.color_rotate_limit;
                    self.colors.rotate_right(1);
                }
                self.rotation.x += 0.5 * time_factor;
                self.rotation.y += 0.5 * time_factor;
                self.rotation.z += 0.5 * time_factor;
                self.faces_in_screen = remap_f32(
                    self.start_time as f32,
                    self.end_time as f32,
                    0_f32,
                    self.faces.len() as f32 / 2.0,
                    self.now_time as f32,
                )
                .clamp(0_f32, self.faces.len() as f32 / 2.0);
            }
            Sequence::Torus04Rotate => {
                self.color_rotate_milis += t;
                if self.color_rotate_milis > self.color_rotate_limit {
                    self.color_rotate_milis -= self.color_rotate_limit;
                    self.colors.rotate_right(1);
                }
                self.rotation.x += 0.5 * time_factor;
                self.rotation.y += 0.5 * time_factor;
                self.rotation.z += 0.5 * time_factor;
            }
            Sequence::Torus04Out => {
                self.color_rotate_milis += t;
                if self.color_rotate_milis > self.color_rotate_limit {
                    self.color_rotate_milis -= self.color_rotate_limit;
                    self.colors.rotate_right(1);
                }
                self.rotation.x += 0.5 * time_factor;
                self.rotation.y += 0.5 * time_factor;
                self.rotation.z += 0.5 * time_factor;
                self.faces_in_screen = remap_f32(
                    self.start_time as f32,
                    self.end_time as f32,
                    self.faces.len() as f32 / 2.0,
                    0_f32,
                    self.now_time as f32,
                )
                .clamp(0_f32, self.faces.len() as f32 / 2.0)
            }

            _ => return,
        }

        // let camera = Vec3 {
        //     x: 0.0_f32,
        //     y: 0.0_f32,
        //     z: 5.0_f32,
        // };

        // self.rotation.x += 0.5 * time_factor;
        // self.rotation.y += 0.5 * time_factor;
        // self.rotation.z += 0.5 * time_factor;

        // clean Vec of transformed 3D points
        self.transformed_3dpoints.clear();

        for point in self.vertices.iter() {
            // apply rotation
            let mut rotated_point = *point;
            rotated_point.rotate_x(self.rotation.x);
            rotated_point.rotate_y(self.rotation.y);
            rotated_point.rotate_z(self.rotation.z);

            self.transformed_3dpoints
                .push(rotated_point.add(&self.camera));

            // apply translation?
        }
        //self.transformed_3dpoints.sort_unstable_by(|l, r| r.z.total_cmp(&l.z)); // order 3dpoints by z after trasformation
        // recalculate z for the faces and reorder them
        self.faces.iter_mut().for_each(|torus_face| {
            // get transformed coordinates for the points 2 and 3 of the triangle 1
            let z1 = self.transformed_3dpoints[torus_face.t1.v2].z;
            let z2 = self.transformed_3dpoints[torus_face.t1.v3].z;
            // calculate average z
            let av_z = (z1 + z2) / 2.0_f32;
            torus_face.z = av_z;
        });
        // order faces by the average z
        self.faces.sort_unstable_by(|l, r| r.z.total_cmp(&l.z));

        // project the transformed_3dpoints to the screen
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

        // build the pixel queue
        // get center of streaming texture
        let center = Vec2 {
            x: (WIDTH as f32 / 2.0_f32),
            y: (HEIGHT as f32 / 2.0_f32),
        };

        // rotate colors
        //self.colors.rotate_right(1);

        for face in self.faces.iter().take(self.faces_in_screen as usize) {
            // TODO: change this to .for_each(||)
            let color: [u8; 4] = self.colors[face.color].to_be_bytes();
            //let a = color[0];
            let r = color[1];
            let g = color[2];
            let b = color[3];

            let t1 = &face.t1;
            let t2 = &face.t2;

            let point0 = &self.screen_points[t1.v1].v.add(&center);
            let point1 = &self.screen_points[t1.v2].v.add(&center);
            let point2 = &self.screen_points[t1.v3].v.add(&center);
            display.filled_triangle_queue(&mut self.pixel_queue, point0, point1, point2, r, g, b);
            let x0 = point0.x as i32;
            let y0 = point0.y as i32;
            let x1 = point1.x as i32;
            let y1 = point1.y as i32;
            display.line_queue(&mut self.pixel_queue, x0, y0, x1, y1, 0_u8, 0_u8, 0_u8);
            let x1 = point2.x as i32;
            let y1 = point2.y as i32;
            display.line_queue(&mut self.pixel_queue, x0, y0, x1, y1, 0_u8, 0_u8, 0_u8);

            let point0 = &self.screen_points[t2.v1].v.add(&center);
            let point1 = &self.screen_points[t2.v2].v.add(&center);
            let point2 = &self.screen_points[t2.v3].v.add(&center);
            display.filled_triangle_queue(&mut self.pixel_queue, point0, point1, point2, r, g, b);
            let x0 = point2.x as i32;
            let y0 = point2.y as i32;
            let x1 = point0.x as i32;
            let y1 = point0.y as i32;
            display.line_queue(&mut self.pixel_queue, x0, y0, x1, y1, 0_u8, 0_u8, 0_u8);
            let x1 = point1.x as i32;
            let y1 = point1.y as i32;
            display.line_queue(&mut self.pixel_queue, x0, y0, x1, y1, 0_u8, 0_u8, 0_u8);
        }
    }

    // implement render
    fn render(&self, display: &mut Display) {
        display.clear_streaming_buffer("torus", 0, 0, 0);
        display.put_pixel_queue("torus", &self.pixel_queue);
        display.streaming_buffer_to_canvas("torus");
    }
}
