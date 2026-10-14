use crate::vector::Vec2;

#[derive(Debug)]
pub struct Face {
    pub a: i32,
    pub b: i32,
    pub c: i32
}

#[derive(Debug, Default)]
pub struct Triangle {
    pub point: [Vec2; 3]
}


