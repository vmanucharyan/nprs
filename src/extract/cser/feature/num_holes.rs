use image::Image;
use structures::Point;
use extract::cser::Incremental;
use extract::cser::feature::Feature;

type Matrix = [[bool;3];3];

#[derive(Debug, Clone)]
pub struct NumHoles {
    genus: f32,
    reg_idx: usize
}

impl Incremental for NumHoles {
    fn init(_: Point, reg_idx: usize) -> Self {
        NumHoles { genus: 0.0f32, reg_idx: reg_idx }
    }

    fn increment(&mut self, p: Point,  _: &Image<u8>, reg_image: &Image<Option<usize>>) {
        let mut m: Matrix = [[false;3];3];

        for x in -1..2 {
            for y in -1..2 {
                let q = Point { x: p.x + y, y: p.y + y };
                if reg_image.inside(q.x, q.y) {
                    m[(x + 1) as usize][(y + 1) as usize] = reg_image[(q.x, q.y)] == Some(self.reg_idx);
                }
            }
        }

        m[1][1] = false;
        let (bc1, bc3, bcd) = count_patterns(&m);

        m[1][1] = true;
        let (ac1, ac3, acd) = count_patterns(&m);

        let dc1 = bc1 - ac1;
        let dc3 = bc3 - ac3;
        let dcd = bcd - acd;

        self.genus = 0.25f32 * ((dc1 - dc3 + 2 * dcd) as f32);
    }

    fn merge(&mut self, other: &Self, _: &Image<u8>, _: &Image<Option<usize>>) {
        self.genus += other.genus;
    }
}

impl Feature for NumHoles {
    fn value(&self, out: &mut Vec<f32>) {
        out.push(1.0f32 - self.genus);
    }
}

fn count_patterns(m: &Matrix) -> (i32, i32, i32) {
    let mut c1 = 0;
    let mut c3 = 0;
    let mut cd = 0;

    for x in 0..2 {
        for y in 0..2 {
            let mut count = 0;

            count += if m[x][y] { 1 } else { 0 };
            count += if m[x + 1][y] { 1 } else { 0 };
            count += if m[x][y + 1] { 1 } else { 0 };
            count += if m[x + 1][y + 1] { 1 } else { 0 };

            match count {
                1 => c1 += 1,
                3 => c3 += 1,
                2 if (m[x + 1][y + 1] && m[x][y]) || (m[x + 1][y] && m[x][y + 1]) => {
                    cd += 1
                },
                _ => {}
            }
        }
    }

    (c1, c3, cd)
}

#[test]
fn count_patterns_test_1() {

    // +-----+--+
    // |xx|  |  |    c1: 1
    // +--------+    c3: 2
    // |xx|xx|  |    cd: 1
    // +--------+
    // |xx|  |xx|
    // +---------

    let m = [
        [true , false, false],
        [true , true , false],
        [true , false, true ]
    ];
    let expected = (1, 2, 1);

    let actual = count_patterns(&m);

    assert_eq!(actual, expected);
}

#[test]
fn count_patterns_test_2() {

    // +-----+--+
    // |  |  |  |    c1: 0
    // +--------+    c3: 0
    // |  |  |  |    cd: 0
    // +--------+
    // |  |  |  |
    // +---------

    let m = [
        [false, false, false],
        [false, false, false],
        [false, false, false]
    ];
    let expected = (0, 0, 0);

    let actual = count_patterns(&m);

    assert_eq!(actual, expected);
}

#[test]
fn count_patterns_test_3() {

    // +-----+--+
    // |xx|  |  |    c1: 2
    // +--------+    c3: 0
    // |  |xx|  |    cd: 2
    // +--------+
    // |  |  |xx|
    // +---------

    let m = [
        [true , false, false],
        [false, true , false],
        [false, false, true ]
    ];
    let expected = (2, 0, 2);

    let actual = count_patterns(&m);

    assert_eq!(actual, expected);
}

#[test]
fn count_patterns_test_4() {

    // +-----+--+
    // |  |xx|xx|    c1: 1
    // +--------+    c3: 1
    // |  |xx|  |    cd: 1
    // +--------+
    // |  |  |xx|
    // +---------

    let m = [
        [false, true , true ],
        [false, true , false],
        [false, false, true ]
    ];
    let expected = (1, 1, 1);

    let actual = count_patterns(&m);

    assert_eq!(actual, expected);
}

#[test]
fn count_patterns_test_5() {

    // +-----+--+
    // |xx|xx|xx|    c1: 0
    // +--------+    c3: 0
    // |xx|xx|xx|    cd: 0
    // +--------+
    // |xx|xx|xx|
    // +---------

    let m = [
        [true , true , true ],
        [true , true , true ],
        [true , true , true ]
    ];
    let expected = (0, 0, 0);

    let actual = count_patterns(&m);

    assert_eq!(actual, expected);
}
