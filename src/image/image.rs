use std::ops::Index;

#[derive(Clone, PartialEq, Eq)]
pub struct Image<T: Clone + Copy> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Copy + Clone> Image<T> {
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

    pub fn inside(&self, x: i32, y: i32) -> bool {
        x < self.width as i32 &&  x >= 0 &&
        y < self.height as i32 && y >= 0
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, value: T) {
        debug_assert!(self.inside(x, y));
        unsafe {
            let mut px = self.data.get_unchecked_mut((y as usize) * self.width + (x as usize));
            *px = value;
        }
    }

    pub fn map<F, A: Copy + Clone>(&self, fun: F) -> Image<A>
    where F: Fn(&T) -> A {
        Image {
            width: self.width,
            height: self.height,
            data: self.data.iter().map(|b| fun(b)).collect()
        }
    }

    pub fn data<'a>(&'a self) -> &'a [T] {
        &self.data[..]
    }
}

impl<T: Copy + Clone> Index<(usize, usize)> for Image<T> {
    type Output = T;

    fn index(&self, p: (usize, usize)) -> &T {
        let (x, y) = p;
        debug_assert!(self.inside(x as i32, y as i32));
        unsafe {
            self.data.get_unchecked(y * self.width + x)
        }
    }
}

impl<T: Copy + Clone> Index<(i32, i32)> for Image<T> {
    type Output = T;

    fn index(&self, p: (i32, i32)) -> &T {
        let (x, y) = p;
        debug_assert!(self.inside(x, y));
        unsafe {
            self.data.get_unchecked((y as usize) * self.width + (x as usize))
        }
    }
}
