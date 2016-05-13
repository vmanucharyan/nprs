pub use super::detector::*;
pub use image::Image;
pub use structures::{Point, Rect};
pub use extract::cser::{Incremental, Region, HasPoints};
pub use extract::cser::feature::AspectRatio;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestInc {
    points: Vec<Point>
}

impl Incremental for TestInc {
    fn init(p: Point) -> Self {
        TestInc { points: vec![p] }
    }

    fn increment(&mut self, p: Point) {
        self.points.push(p);
    }

    fn merge(&mut self, r: &TestInc) {
        self.points.extend_from_slice(&r.points[..]);
    }
}

impl HasPoints for TestInc {
    fn points<'a>(&'a self) -> &'a [Point] {
        &self.points[..]
    }
}

describe! detect_regions {
    describe! hist {
        it "should return points for each intensity level from 0 to 255" {
            let data = vec![0, 6, 233, 6, 13, 200, 13, 13];
            let image: Image<u8> = Image::from_data(data, 4, 2);
            let hist = hist(&image);

            assert_eq!(hist.len(), 255);
            assert_eq!(hist[0].len(), 1);
            assert_eq!(hist[13].len(), 3);
            assert_eq!(hist[6].len(), 2);
        }
    }

    describe! find_neighbors {
        it "should return indexes of adjacent regions" {
            let r: Region = Incremental::init(Point { x: 0, y: 0 });
            let b: Vec<u8> = vec![
                0, 1, 2, 0,
                0, 0, 2, 0,
                0, 4, 0, 5,
                0, 0, 6, 7,
            ];

            let data = b.iter()
                .map(|x| if x.clone() != 0u8 { Some(x.clone() as usize) } else { None })
                .collect();

            let img: Image<Option<usize>> = Image::from_data(data, 4, 4);
            let reg: Vec<Region> = vec![];

            assert_eq!(find_neighbors(&img, Point { x: 0, y: 0 }).len(), 1);
            assert_eq!(find_neighbors(&img, Point { x: 2, y: 2 }).len(), 4);
            assert_eq!(find_neighbors(&img, Point { x: 3, y: 3 }).len(), 2);
            assert_eq!(find_neighbors(&img, Point { x: 3, y: 1 }).len(), 2);

            let expected_points = vec![5, 6];
            assert_eq!(find_neighbors(&img, Point { x: 3, y: 3 }), expected_points);
        }
    }

    describe! process_point {
        before_each {
            let b: Vec<u8> = vec![
                0, 1, 1, 0, 0, 0,
                0, 0, 1, 0, 0, 0,
                2, 2, 0, 3, 0, 0,
                0, 0, 0, 3, 0, 0,
            ];

            let data = b.iter()
                .map(|x| if x.clone() != 0u8 { Some((x - 1) as usize) } else { None })
                .collect();

            let mut reg_img: Image<Option<usize>> = Image::from_data(data, 6, 4);
            let r1 = TestInc {
                points: vec![
                    Point { x: 1, y: 0 },
                    Point { x: 2, y: 0 },
                    Point { x: 2, y: 1 },
                ]
            };

            let r2 = TestInc {
                points: vec![
                    Point { x: 0, y: 2 },
                    Point { x: 1, y: 2 }
                ]
            };

            let r3 = TestInc {
                points: vec![
                    Point { x: 3, y: 2 },
                    Point { x: 3, y: 3 }
                ]
            };

            let mut regions: Vec<TestInc> = vec![r1.clone(), r2.clone(), r3.clone()];
        }

        it "should create new region and add it to regions list if there are no adjacent regions" {
            let new_point = Point { x: 5, y: 0 };
            let expected_region = TestInc {
                points: vec![new_point]
            };

            process_point::<TestInc>(new_point, &mut reg_img, &mut regions);
            assert_eq!(*regions.last().unwrap(), expected_region);
        }

        it "should create new region and draw it on region image if there are no adjacent regions" {
            let b: Vec<u8> = vec![
                0, 1, 1, 0, 0, 4,
                0, 0, 1, 0, 0, 0,
                2, 2, 0, 3, 0, 0,
                0, 0, 0, 3, 0, 0,
            ];
            let expected_data: Vec<Option<usize>> = b.iter()
                .map(|x| if x.clone() != 0u8 { Some((x - 1) as usize) } else { None })
                .collect();

            process_point::<TestInc>(Point { x: 5, y: 0 }, &mut reg_img, &mut regions);

            assert_eq!(reg_img.data(), &expected_data[..]);
        }

        it "should increment region and draw it on regions image if there is only one adjacent region" {
            let b: Vec<u8> = vec![
                1, 1, 1, 0, 0, 0,
                0, 0, 1, 0, 0, 0,
                2, 2, 0, 3, 0, 0,
                0, 0, 0, 3, 0, 0,
            ];
            let expected_data: Vec<Option<usize>> = b.iter()
                .map(|x| if x.clone() != 0u8 { Some((x - 1) as usize) } else { None })
                .collect();

            process_point::<TestInc>(Point { x: 0, y: 0 }, &mut reg_img, &mut regions);

            assert_eq!(reg_img.data(), &expected_data[..]);
            assert_eq!(regions[0].points().len(), 4);
        }

        it "should merge regions if there are 2 adjacent regions" {
            let b: Vec<u8> = vec![
                0, 1, 1, 0, 0, 0,
                0, 1, 1, 0, 0, 0,
                1, 1, 0, 3, 0, 0,
                0, 0, 0, 3, 0, 0,
            ];
            let expected_data: Vec<Option<usize>> = b.iter()
                .map(|x| if x.clone() != 0u8 { Some((x - 1) as usize) } else { None })
                .collect();

            process_point::<TestInc>(Point { x: 1, y: 1 }, &mut reg_img, &mut regions);

            assert_eq!(reg_img.data(), &expected_data[..]);
        }

        it "should merge all regions if there are more than 2 adjacent regions" {
            let b: Vec<u8> = vec![
                0, 1, 1, 0, 0, 0,
                0, 0, 1, 0, 0, 0,
                1, 1, 1, 1, 0, 0,
                0, 0, 0, 1, 0, 0,
            ];
            let expected_data: Vec<Option<usize>> = b.iter()
                .map(|x| if x.clone() != 0u8 { Some((x - 1) as usize) } else { None })
                .collect();

            process_point::<TestInc>(Point { x: 2, y: 2 }, &mut reg_img, &mut regions);

            assert_eq!(reg_img.data(), &expected_data[..]);
        }
    }

    describe! index_twice {
        it "should return references to indexed elements" {
            let mut sl = [1, 2, 3, 4];

            if let Some((a, b)) = index_twice(&mut sl, 1, 3) {
                assert_eq!(*a, 2);
                assert_eq!(*b, 4);
            } else {
                panic!("got None");
            }
        }
    }
}
