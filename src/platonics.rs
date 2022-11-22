use crate::{point::Point, vector::Vec3};

#[derive(Debug)]
pub struct Platonics {
    tetra: Vec<Vec3>,
    octa: Vec<Vec3>,
    cube: Vec<Vec3>,
    //dodec: Vec<Vec3>,
    //icos: Vec<Vec3>,
    screen_points: Vec<Point>,
    rotation: Vec3,
}

impl Platonics {
    pub fn new() -> Self {
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
                x: 1_f32,
                y: 1_f32,
                z: 1_f32,
            },
            Vec3 {
                x: -1_f32,
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
                y: -1_f32,
                z: 1_f32,
            },
            Vec3 {
                x: 1_f32,
                y: 1_f32,
                z: -1_f32,
            },
            Vec3 {
                x: -1_f32,
                y: 1_f32,
                z: -1_f32,
            },
            Vec3 {
                x: 1_f32,
                y: -1_f32,
                z: -1_f32,
            },
            Vec3 {
                x: -1_f32,
                y: -1_f32,
                z: -1_f32,
            },
        ];

        Self {
            tetra,
            octa,
            cube,
            //dodec,
            //icos,
            screen_points: Vec::<Point>::new(),
            rotation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}
