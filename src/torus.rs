use std::f32::consts::{PI, TAU};

use crate::{display::Display, point::Point, triangle::Triangle, vector::Vec3};

const VS_SEGMENT: usize = 16; // vertices per segment
const SEGMENTS: usize = 32; // number of segments
const S_RADIUS: f32 = 0.25; // radius of segment
const T_RADIUS: f32 = 0.75; // center of torus to center of segment radius

pub struct Torus {
    vertices: Vec<Vec3>,
    transformed_3dpoints: Vec<Vec3>,
    triangles: Vec<Triangle>,
    screen_points: Vec<Point>,
    rotation: Vec3,
}

/* impl Default for Starfield {
    fn default() -> Self {
        Self::new()
    }
} */

impl Torus {
    pub fn new(display: &mut Display) -> Self {
        // create stream_buffer in display
        display.add_streaming_buffer("torus", 640, 360);
        // calculate torus vertices
        let mut vertices_torus = Vec::<Vec3>::new();
        let mut vertices_segment = Vec::<Vec3>::new();

        let angle_in_segment = TAU / VS_SEGMENT as f32; // TAU = PI*2
        let angle_between_segments = TAU / SEGMENTS as f32;
        let segment_radius = Vec3 {
            x: 0.0,
            y: S_RADIUS,
            z: 0.0,
        };
        let torus_radius = Vec3 {
            x: T_RADIUS,
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
        for _ in 0..VS_SEGMENT {
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
        for segment in 0..SEGMENTS {
            // copy
            let mut clone_segment = vertices_segment.clone();
            //rotate
            clone_segment
                .iter_mut()
                .for_each(|v| v.rotate_y(angle_between_segments * segment as f32));
            //append to list of vertices
            vertices_torus.append(&mut clone_segment);
        }
        let size = vertices_torus.len();
        // caluclate torus faces on the vertices
        Self {
            vertices: vertices_torus,
            transformed_3dpoints: Vec::with_capacity(size),
            triangles: Vec::new(),
            screen_points: Vec::with_capacity(size),
            rotation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }

    // implement update
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

        for point in self.vertices.iter() {
            // apply rotation
            let mut rotated_point = *point;
            rotated_point.rotate_x(self.rotation.x);
            rotated_point.rotate_y(self.rotation.y);
            rotated_point.rotate_z(self.rotation.z);

            self.transformed_3dpoints.push(rotated_point.add(&camera));
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

    // implement render
    pub fn render(&self, display: &mut Display) {
        display.clear_streaming_buffer("torus", 0, 0, 0);
        let width = display.streaming_buffer_width("torus").unwrap();
        let height = display.streaming_buffer_height("torus").unwrap();
        for point in self.screen_points.iter() {
            let x: i32 = (point.v.x.round() + (width as f32 / 2.0_f32)) as i32; // TODO: change this to w_width and w_height
            let y: i32 = (point.v.y.round() + (height as f32 / 2.0_f32)) as i32;
            display.put_pixel("torus", x, y, point.r, point.g, point.b);
            //let size_factor = 3_f32 / point.z;
            //display.put_sprite_centered("particle02", x, y, size_factor);
        }
        display.streaming_buffer_to_canvas("torus");
    }
}
