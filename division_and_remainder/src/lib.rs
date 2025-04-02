pub struct Rem (pub i32, pub i32);

pub fn divide(x: i32, y: i32) -> (i32, i32) {
    (x / y, x % y)
}