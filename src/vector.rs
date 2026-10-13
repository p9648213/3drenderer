#[derive(Clone, Copy, Debug, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn vec3_rotate_x(v: Vec3, angle: f32) -> Vec3 {
    Vec3 {
        x: v.x,
        y: v.y *  angle.cos() - v.z * angle.sin(),
        z: v.y * angle.sin() + v.z * angle.cos()
    }
}

pub fn vec3_rotate_y(v: Vec3, angle: f32) -> Vec3 {
    Vec3 {
        x: v.x * angle.cos() - v.z * angle.sin(),
        y: v.y,
        z: v.x * angle.sin() + v.z * angle.cos()
    }
}

pub fn vec3_rotate_z(v: Vec3, angle: f32) -> Vec3 {
    Vec3 {
        x: v.x * angle.cos() - v.y * angle.sin(),
        y: v.x * angle.sin() + v.y * angle.cos(),
        z: v.z
    }
}


