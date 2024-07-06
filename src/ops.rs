use std::ops::*;
use crate::vec::*;

/*****************************************************************************/
impl<T: Numerical + Add<Output = T>> Add for Vector2<T> {
    type Output = Vector2<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}
impl<T: Numerical + Sub<Output = T>> Sub for Vector2<T> {
    type Output = Vector2<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}
impl<T: Numerical + Mul<Output = T>> Mul for Vector2<T> {
    type Output = Vector2<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}


/*****************************************************************************/
impl<T: Numerical + Add<Output = T>> Add for Vector3<T> {
    type Output = Vector3<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl<T: Numerical + Sub<Output = T>> Sub for Vector3<T> {
    type Output = Vector3<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl<T: Numerical + Mul<Output = T>> Mul for Vector3<T> {
    type Output = Vector3<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}


/*****************************************************************************/