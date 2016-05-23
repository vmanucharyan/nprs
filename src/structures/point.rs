#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, RustcEncodable, RustcDecodable)]
pub struct Point {
    pub x: i32,
    pub y: i32
}
