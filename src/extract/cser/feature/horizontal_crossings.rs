use std::cmp::{min, max};
use std::collections::VecDeque;

use image::Image;
use structures::Point;
use extract::cser::{Feature, Incremental};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HorizontalCrossings {
    num_crossings: VecDeque<i32>,
    y_top: i32,
    y_btm: i32,
    reg_idx: usize
}

impl Incremental for HorizontalCrossings {
    fn init(p: Point, reg_idx: usize) -> Self {
        let mut nc: VecDeque<i32> = VecDeque::new();
        nc.push_back(2);

        HorizontalCrossings {
            num_crossings: nc,
            y_top: p.y,
            y_btm: p.y,
            reg_idx: reg_idx
        }
    }

    fn increment(&mut self, p: Point,  _: &Image<u8>, reg_image: &Image<Option<usize>>) {
        let mut transitions = 0;

        if (reg_image.inside(p.x - 1, p.y)) && (reg_image[(p.x - 1, p.y)] == Some(self.reg_idx)) {
            transitions += 1;
        } else {
            transitions -= 1;
        }

        if (reg_image.inside(p.x + 1, p.y)) && (reg_image[(p.x + 1, p.y)] == Some(self.reg_idx)) {
            transitions += 1;
        } else {
            transitions -= 1;
        }

        if p.y < self.y_top {
            self.y_top = p.y;
            self.num_crossings.push_front(transitions);
        }
        else if p.y > self.y_btm {
            self.y_btm = p.y;
            self.num_crossings.push_back(transitions);
        }
        else {
            self.num_crossings[(p.y - self.y_top) as usize] += transitions;
        }
    }

    fn merge(&mut self, other: &Self, _: &Image<u8>, _: &Image<Option<usize>>) {
        let y_top = min(self.y_top, other.y_top);
        let y_btm = max(self.y_btm, other.y_btm);

        let mut nc: VecDeque<i32> = VecDeque::new();
        nc.resize((y_btm - y_top + 1) as usize, 0);

        for y in y_top...y_btm {
            if y >= self.y_top && y <= self.y_btm {
                nc[(y - y_top) as usize] += self.num_crossings[(y - self.y_top) as usize];
            }

            if y >= other.y_top && y <= other.y_btm {
                nc[(y - y_top) as usize] += other.num_crossings[(y - other.y_top) as usize];
            }
        }

        self.y_top = y_top;
        self.y_btm = y_btm;
        self.num_crossings = nc;
    }
}

impl Feature for HorizontalCrossings {
    fn value(&self, out: &mut Vec<f32>) {
        let res =
            if self.num_crossings.len() == 0 {
                0.0f32
            }
            else if self.num_crossings.len() == 1 {
                self.num_crossings[0] as f32
            }
            else {
                let mut m = [0; 3];
                let l = self.num_crossings.len() as f32;
                m[0] = self.num_crossings[(l * 0.17) as usize];
                m[1] = self.num_crossings[(l * 0.50) as usize];
                m[2] = self.num_crossings[(l * 0.83) as usize];
                m.sort();
                m[1] as f32
            };

        out.push(res);
    }
}

#[test]
fn merge_with_intersection() {

    //         10                13          15
    // +-------x-----x-----x-----x-----x-----x--------->
    //      +-----+-----+-----+-----+
    //      |  2  |  3  |  2  |  1  |
    //      +-----+-----------------------+-----+
    //                  |  4  |  2  |  2  |  2  |
    //                  +-----+-----+-----+-----+

    let mut hc1 = HorizontalCrossings {
        num_crossings: vec![2, 3, 2, 1].into_iter().collect(),
        y_top: 10,
        y_btm: 13,
        reg_idx: 1
    };

    let hc2 = HorizontalCrossings {
        num_crossings: vec![4, 2, 2, 2].into_iter().collect(),
        y_top: 12,
        y_btm: 15,
        reg_idx: 2
    };

    let expected_hc = HorizontalCrossings {
        num_crossings: vec![2, 3, 6, 3, 2, 2].into_iter().collect(),
        y_top: 10,
        y_btm: 15,
        reg_idx: 1
    };

    let img: Image<u8> = Image::from_data(vec![], 0, 0);
    let reg_img: Image<Option<usize>> = Image::from_data(vec![], 0, 0);

    hc1.merge(&hc2, &img, &reg_img);

    assert_eq!(hc1, expected_hc);
}

#[test]
fn merge_no_intersection() {

    //        10                13          15
    // +------x-----x-----x-----x-----x-----x--------->
    //     +-----+-----+-----+-----+
    //     |  2  |  3  |  2  |  1  |
    //     +-----+-----+-----+-----------+-----+
    //                             |  2  |  2  |
    //                             +-----+-----+

    let mut hc1 = HorizontalCrossings {
        num_crossings: vec![2, 3, 2, 1].into_iter().collect(),
        y_top: 10,
        y_btm: 13,
        reg_idx: 1
    };

    let hc2 = HorizontalCrossings {
        num_crossings: vec![2, 2].into_iter().collect(),
        y_top: 14,
        y_btm: 15,
        reg_idx: 2
    };

    let expected_hc = HorizontalCrossings {
        num_crossings: vec![2, 3, 2, 1, 2, 2].into_iter().collect(),
        y_top: 10,
        y_btm: 15,
        reg_idx: 1
    };

    let img: Image<u8> = Image::from_data(vec![], 0, 0);
    let reg_image: Image<Option<usize>> = Image::from_data(vec![], 0, 0);

    hc1.merge(&hc2, &img, &reg_image);

    assert_eq!(hc1, expected_hc);
}

#[test]
fn merge_included() {

    //        10                            15
    // +------x-----x-----x-----x-----x-----x--------->
    //     +-----+-----+-----+-----+-----+-----+
    //     |  2  |  3  |  2  |  4  |  6  |  2  |
    //     +-----+-----------------------------+
    //                 |  4  |  2  |  3  |
    //                 +-----+-----+-----+

    let mut hc1 = HorizontalCrossings {
        num_crossings: vec![2, 3, 2, 4, 6, 2].into_iter().collect(),
        y_top: 10,
        y_btm: 15,
        reg_idx: 1
    };

    let hc2 = HorizontalCrossings {
        num_crossings: vec![4, 2, 3].into_iter().collect(),
        y_top: 12,
        y_btm: 14,
        reg_idx: 2
    };

    let expected_hc = HorizontalCrossings {
        num_crossings: vec![2, 3, 6, 6, 9, 2].into_iter().collect(),
        y_top: 10,
        y_btm: 15,
        reg_idx: 1
    };

    let img: Image<u8> = Image::from_data(vec![], 0, 0);
    let reg_image: Image<Option<usize>> = Image::from_data(vec![], 0, 0);

    hc1.merge(&hc2, &img, &reg_image);

    assert_eq!(hc1, expected_hc);
}
