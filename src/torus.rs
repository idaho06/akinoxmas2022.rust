use std::f32::consts::{PI, TAU};

use crate::{
    display::Display,
    point::{Pixel, Point},
    triangle::Triangle,
    vector::{Vec2, Vec3},
};

const VS_SEGMENT: usize = 8; // vertices per segment
const SEGMENTS: usize = 14; // number of segments
const S_RADIUS: f32 = 0.25; // radius of segment
const T_RADIUS: f32 = 0.75; // center of torus to center of segment radius

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
}

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

impl Torus {
    pub fn new(display: &mut Display) -> Self {
        // colors
        let colors = vec![
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
        ]; //ARGB888];
           // create stream_buffer in display
        display.add_streaming_buffer("torus", WIDTH, HEIGHT);
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

        // calculate torus faces on the vertices
        let mut faces = Vec::<TorusFace>::new();

        let torus_width = SEGMENTS;
        let torus_height = VS_SEGMENT;
        let torus_size = SEGMENTS * VS_SEGMENT;
        let mut v1_index = 0_usize;
        let mut v2_index = 0_usize;
        let mut v3_index = 0_usize;
        let mut v4_index = 0_usize;

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

                let face = TorusFace { t1, t2, z: 0_f32, color: color%16};
                faces.push(face);
                color += 1;
            }
            v1_index = (i * torus_height) + torus_height - 1;
            v2_index = (i * torus_height);
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
            let face = TorusFace { t1, t2, z: 0_f32, color: color%16};
            faces.push(face);
            color += 1;
        }

        Self {
            vertices: vertices_torus,
            transformed_3dpoints: Vec::with_capacity(size),
            faces,
            screen_points: Vec::with_capacity(size),
            pixel_queue: Vec::<Pixel>::new(),
            rotation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            colors,
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
        self.pixel_queue.truncate(0);

        // get center of streaming texture
        let center = Vec2 {
            x: (WIDTH as f32 / 2.0_f32),
            y: (HEIGHT as f32 / 2.0_f32),
        };

        // rotate colors
        //self.colors.rotate_right(1);

        for face in self.faces.iter() { // TODO: change this to .for_each(||)
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
    pub fn render(&self, display: &mut Display) {
        display.clear_streaming_buffer("torus", 0, 0, 0);
        display.put_pixel_queue("torus", &self.pixel_queue);
        display.streaming_buffer_to_canvas("torus");
    }
}
