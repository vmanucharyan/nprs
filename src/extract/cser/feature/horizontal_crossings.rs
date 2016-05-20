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
    fn init(p: Point, reg_idx: usize, _: i32) -> Self {
        let mut nc: VecDeque<i32> = VecDeque::new();
        nc.push_back(2);

        HorizontalCrossings {
            num_crossings: nc,
            y_top: p.y,
            y_btm: p.y,
            reg_idx: reg_idx
        }
    }

    fn increment(&mut self, p: Point, _: i32,   _: &Image<u8>,  reg_image: &Image<Option<usize>>) {
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

    fn merge(&mut self, other: &HorizontalCrossings, _: &Image<u8>, _: &Image<Option<usize>>) {
        // intersection area
        {
            let a = max(self.y_top, other.y_top);
            let b = min(self.y_btm, other.y_btm);
            if b >= a {
                for y in a...b {
                    let is = y - self.y_top;
                    let io = y - other.y_top;
                    debug_assert!(is >= 0 && io >= 0);
                    self.num_crossings[is as usize] += other.num_crossings[io as usize];
                }
            }
        };

        // add to top
        {
            let d = self.y_top - other.y_top;
            if d > 0 {
                for i in (0..d).rev() {
                    self.num_crossings.push_front(other.num_crossings[i as usize]);
                }
            }
        };
        self.y_top = min(self.y_top, other.y_top);

        // add to bottom
        {
            let d = other.y_btm - self.y_btm;
            if d > 0 {
                for i in (0..d).rev() {
                    let io = other.num_crossings.len() as i32 - i - 1;
                    debug_assert!(io >= 0);
                    self.num_crossings.push_back(other.num_crossings[io as usize]);
                }
            }
        };
        self.y_btm = max(self.y_btm, other.y_btm);
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
fn merge_no_intersection_reverse() {

    //        10                13          15
    // +------x-----x-----x-----x-----x-----x--------->
    //     +-----+-----+-----+-----+
    //     |  2  |  3  |  2  |  1  |
    //     +-----+-----+-----+-----------+-----+
    //                             |  2  |  2  |
    //                             +-----+-----+

    let hc1 = HorizontalCrossings {
        num_crossings: vec![2, 3, 2, 1].into_iter().collect(),
        y_top: 10,
        y_btm: 13,
        reg_idx: 1
    };

    let mut hc2 = HorizontalCrossings {
        num_crossings: vec![2, 2].into_iter().collect(),
        y_top: 14,
        y_btm: 15,
        reg_idx: 2
    };

    let expected_hc = HorizontalCrossings {
        num_crossings: vec![2, 3, 2, 1, 2, 2].into_iter().collect(),
        y_top: 10,
        y_btm: 15,
        reg_idx: 2
    };

    let img: Image<u8> = Image::from_data(vec![], 0, 0);
    let reg_image: Image<Option<usize>> = Image::from_data(vec![], 0, 0);

    hc2.merge(&hc1, &img, &reg_image);

    assert_eq!(hc2, expected_hc);
}

#[test]
fn merge_included() {

    //        10                            15
    // +------x-----x-----x-----x-----x-----x--------->
    //     +-----+-----+-----+-----+-----+-----+
    //     |  2  |  3  |  2  |  4  |  6  |  2  |
    //     +-----+-----+-----+-----+-----+-----+
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

#[test]
fn merge_included_reverse() {

    //        10                            15
    // +------x-----x-----x-----x-----x-----x--------->
    //     +-----+-----+-----+-----+-----+-----+
    //     |  2  |  3  |  2  |  4  |  6  |  2  |
    //     +-----+-----------------------------+
    //                 |  4  |  2  |  3  |
    //                 +-----+-----+-----+

    let hc1 = HorizontalCrossings {
        num_crossings: vec![2, 3, 2, 4, 6, 2].into_iter().collect(),
        y_top: 10,
        y_btm: 15,
        reg_idx: 1
    };

    let mut hc2 = HorizontalCrossings {
        num_crossings: vec![4, 2, 3].into_iter().collect(),
        y_top: 12,
        y_btm: 14,
        reg_idx: 2
    };

    let expected_hc = HorizontalCrossings {
        num_crossings: vec![2, 3, 6, 6, 9, 2].into_iter().collect(),
        y_top: 10,
        y_btm: 15,
        reg_idx: 2
    };

    let img: Image<u8> = Image::from_data(vec![], 0, 0);
    let reg_image: Image<Option<usize>> = Image::from_data(vec![], 0, 0);

    hc2.merge(&hc1, &img, &reg_image);

    assert_eq!(hc2, expected_hc);
}
