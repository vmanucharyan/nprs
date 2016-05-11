use std::ops::Index;

pub struct Image<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Image<T> {
    pub fn from_data(_data: Vec<T>, _width: usize, _height: usize) -> Image<T> {
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

impl<T> Index<(usize, usize)> for Image<T> {
    type Output = T;

    fn index(&self, p: (usize, usize)) -> &T {
        let (x, y) = p;
        return &(self.data[y * self.width + x])
    }
}
