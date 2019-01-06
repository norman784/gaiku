use std::{
    fmt::Debug,
    hash::{
        Hash,
        Hasher,
    },
};

use decorum::{
    hash_float_array,
    Primitive,
};

use num_traits::Float;

#[derive(Debug, Copy, Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Debug> Vec2<T> {
    pub fn to_str(&self) -> String {
        format!("{:?},{:?}", self.x, self.y)
    }
}

impl<T: Copy + Clone> From<[T; 2]> for Vec2<T> {
    fn from(value: [T; 2]) -> Self {
        Vec2 { x:  value[0], y: value[1] }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float + Primitive> Hash for Vec3<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_float_array(&[self.x, self.y, self.z], state);
    }
}

impl<T: Float + Primitive> PartialEq for Vec3<T> {
    fn eq(&self, other: &Vec3<T>) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    fn ne(&self, other: &Vec3<T>) -> bool {
        self.x != other.x || self.y != other.y || self.z != other.z
    }
}

impl<T: Float + Primitive> Eq for Vec3<T> {}

impl<T: Debug> Vec3<T> {
    pub fn to_str(&self) -> String {
        format!("{:?},{:?},{:?}", self.x, self.y, self.z)
    }
}

impl<T: Copy + Clone> From<[T; 3]> for Vec3<T> {
    fn from(value: [T; 3]) -> Self {
        Vec3 { x:  value[0], y: value[1], z: value[2] }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Debug> Vec4<T> {
    pub fn to_str(&self) -> String {
        format!("{:?},{:?},{:?},{:?}", self.x, self.y, self.z, self.w)
    }
}

impl<T: Copy + Clone> From<[T; 4]> for Vec4<T> {
    fn from(value: [T; 4]) -> Self {
        Vec4 { x:  value[0], y: value[1], z: value[2], w: value[3] }
    }
}
