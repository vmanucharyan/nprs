use std::ops::Index;

pub struct Image {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn from_data(_data: Vec<u8>, _width: usize, _height: usize) -> Image {
        assert!(_data.len() == _width * _height);
        return Image {
            data: _data,
            width: _width,
            height: _height
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        return (self.width, self.height);
    }

    pub fn width(&self) -> usize {
        return self.width;
    }

    pub fn height(&self) -> usize {
        return self.height;
    }
}

impl Index<(usize, usize)> for Image {
    type Output = u8;

    fn index(&self, p: (usize, usize)) -> &u8 {
        let (x, y) = p;
        return &(self.data[y * self.width + x])
    }
}
