use nalgebra::{Vector2, Vector3};

pub type IVec2 = Vector2<i64>;
pub type IVec3 = Vector3<i64>;

pub const fn vec2<T>(x: T, y: T) -> Vector2<T> {
    Vector2::new(x, y)
}

pub const fn vec3<T>(x: T, y: T, z: T) -> Vector3<T> {
    Vector3::new(x, y, z)
}
