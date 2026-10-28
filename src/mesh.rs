use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{triangle::Face, vector::Vec3};

pub const N_CUBE_VERTICES: usize = 8;
pub const N_CUBE_FACES: usize = 12;

pub const CUBE_VERTICES: [Vec3; N_CUBE_VERTICES] = [
    Vec3 {
        x: -1.0,
        y: -1.0,
        z: -1.0,
    },
    Vec3 {
        x: -1.0,
        y: 1.0,
        z: -1.0,
    },
    Vec3 {
        x: 1.0,
        y: 1.0,
        z: -1.0,
    },
    Vec3 {
        x: 1.0,
        y: -1.0,
        z: -1.0,
    },
    Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    },
    Vec3 {
        x: 1.0,
        y: -1.0,
        z: 1.0,
    },
    Vec3 {
        x: -1.0,
        y: 1.0,
        z: 1.0,
    },
    Vec3 {
        x: -1.0,
        y: -1.0,
        z: 1.0,
    },
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
    Face { a: 6, b: 1, c: 4 },
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

    pub fn load_obj_file_data(&mut self, path: &Path) {
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        let mut line_buffer = String::new();

        loop {
            let bytes_read = reader.read_line(&mut line_buffer).unwrap();
            if bytes_read == 0 {
                break;
            }

            if line_buffer.starts_with("v ") {
                let mut vertices = line_buffer.split_whitespace();
                vertices.next();
                self.vertices.push(Vec3 {
                    x: vertices.next().unwrap().parse::<f32>().unwrap(),
                    y: vertices.next().unwrap().parse::<f32>().unwrap(),
                    z: vertices.next().unwrap().parse::<f32>().unwrap(),
                });
            }

            if line_buffer.starts_with("f ") {
                let mut faces = line_buffer.split_whitespace();
                faces.next();
                let mut vertex_index_arr: [i32; 3] = [0, 0, 0];
                (0..3).for_each(|i| {
                    let face: Vec<&str> = faces.next().unwrap().split('/').collect();
                    let vertex_index = face[0].parse::<i32>().unwrap();
                    let _texture_index = face[1].parse::<i32>().unwrap();
                    let _normal_index = face[2].parse::<i32>().unwrap();
                    vertex_index_arr[i] = vertex_index;
                });
                self.faces.push(Face {
                    a: vertex_index_arr[0],
                    b: vertex_index_arr[1],
                    c: vertex_index_arr[2],
                });
            }

            line_buffer.clear();
        }
    }
}
