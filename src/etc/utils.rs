use std::ops::{Sub, Add, AddAssign, SubAssign};

pub fn abs_diff<T: Ord + Sub<Output = T>> (a: T, b: T) -> T {
    if a >= b {
        a - b
    } else {
        b - a
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point(pub i32, pub i32);

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}