use std::{
    cmp::{Ord, Ordering, PartialOrd},
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::{Add, Div, Mul, Sub},
};

use decorum::{hash_float_array, Primitive};

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
        Vec2 {
            x: value[0],
            y: value[1],
        }
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
        Vec3 {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl<T: Float + Primitive + Add<Output = T> + Copy + Clone> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Float + Primitive + Sub<Output = T> + Copy + Clone> Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Float + Primitive + Div<Output = T> + Copy + Clone> Div for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T: Float + Primitive + Div<Output = T> + Copy + Clone> Mul for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T: Float + Primitive> Ord for Vec3<T> {
    fn cmp(&self, other: &Vec3<T>) -> Ordering {
        if self.x < other.x {
            return Ordering::Less;
        } else if self.x > other.x {
            return Ordering::Greater;
        }

        if self.y < other.y {
            return Ordering::Less;
        } else if self.y > other.y {
            return Ordering::Greater;
        }

        if self.z < other.z {
            return Ordering::Less;
        } else if self.z > other.z {
            return Ordering::Greater;
        }

        Ordering::Equal
    }
}

impl<T: Float + Primitive> PartialOrd for Vec3<T> {
    fn partial_cmp(&self, other: &Vec3<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Vec3<T>) -> bool {
        match self.cmp(other) {
            Ordering::Less => true,
            _ => false,
        }
    }

    fn le(&self, other: &Vec3<T>) -> bool {
        match self.cmp(other) {
            Ordering::Less | Ordering::Equal => true,
            _ => false,
        }
    }

    fn gt(&self, other: &Vec3<T>) -> bool {
        match self.cmp(other) {
            Ordering::Greater => true,
            _ => false,
        }
    }

    fn ge(&self, other: &Vec3<T>) -> bool {
        match self.cmp(other) {
            Ordering::Greater | Ordering::Equal => true,
            _ => false,
        }
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
        Vec4 {
            x: value[0],
            y: value[1],
            z: value[2],
            w: value[3],
        }
    }
}
