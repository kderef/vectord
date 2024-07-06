use std::ops::{Add, Div, Mul, Sub};

pub trait Numerical: Add + Sub + Div + Mul + Copy + Default {}

/// Vector of components `x, y`
#[derive(Default, Debug, Clone, Copy)]
pub struct Vector2<T: Numerical> {
    pub x: T,
    pub y: T
}

/// Vector of components `x, y, z`
#[derive(Default, Debug, Clone, Copy)]
pub struct Vector3<T: Numerical> {
    pub x: T,
    pub y: T,
    pub z: T
}

/// Vector of components `x, y, z, w`
#[derive(Default, Debug, Clone, Copy)]
pub struct Vector4<T: Numerical> {
    x: T,
    y: T,
    z: T,
    w: T
}

/*************************************************************/
impl<T: Numerical> Vector2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
    pub const fn from_scalar(sc: T) -> Self {
        Self {
            x: sc, y: sc,
        }
    }
}
impl<T: Numerical> Vector3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self {
            x, y, z
        }
    }
    pub const fn from_scalar(sc: T) -> Self {
        Self {
            x: sc, y: sc, z: sc,
        }
    }
}
impl<T: Numerical> Vector4<T> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self {
            x, y, z, w
        }
    }
    pub const fn from_scalar(sc: T) -> Self {
        Self {
            x: sc, y: sc, z: sc, w: sc
        }
    }
}