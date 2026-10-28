use crate::{triangle::Face, vector::Vec3};

pub const N_CUBE_VERTICES: usize = 8;
pub const N_CUBE_FACES: usize = 12;

pub const CUBE_VERTICES: [Vec3; N_CUBE_VERTICES]= [
    Vec3 { x: -1.0, y: -1.0, z: -1.0 },
    Vec3 { x: -1.0, y: 1.0, z: -1.0 },
    Vec3 { x: 1.0, y: 1.0, z: -1.0 },
    Vec3 { x: 1.0, y: -1.0, z: -1.0 },
    Vec3 { x: 1.0, y: 1.0, z: 1.0 },
    Vec3 { x: 1.0, y: -1.0, z: 1.0 },
    Vec3 { x: -1.0, y: 1.0, z: 1.0 },
    Vec3 { x: -1.0, y: -1.0, z: 1.0 }
];

pub const CUBE_FACES: [Face; N_CUBE_FACES] = [
    Face { a: 1, b: 2, c: 3 },
    Face { a: 1, b: 3, c: 4 },
    Face { a: 4, b: 3, c: 5 },
    Face { a: 4, b: 5, c: 6 },
    Face { a: 6, b: 5, c: 7 },
    Face { a: 6, b: 7, c: 8 },
    Face { a: 8, b: 7, c: 2 },
    Face { a: 8, b: 2, c: 1 },
    Face { a: 2, b: 7, c: 5 },
    Face { a: 2, b: 5, c: 3 },
    Face { a: 6, b: 8, c: 1 },
    Face { a: 6, b: 1, c: 4 }
];

#[derive(Debug, Default)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Face>,
    pub rotation: Vec3,
}

impl Mesh {
    pub fn load_cube_mesh_data(&mut self) {
        (0..N_CUBE_VERTICES).for_each(|i| {
            self.vertices.push(CUBE_VERTICES[i]);
        });
        (0..N_CUBE_FACES).for_each(|i| {
            self.faces.push(CUBE_FACES[i]);
        });
    }
}
