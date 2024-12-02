use std::ops::Sub;

pub fn abs_diff<T: Ord + Sub<Output = T>> (a: T, b: T) -> T {
    if a >= b {
        a - b
    } else {
        b - a
    }
}