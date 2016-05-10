pub struct Image {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn at(&self, x: usize, y: usize) -> u8 {
        return self.data[y * self.width + x];
    }
}
