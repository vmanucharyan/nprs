pub use super::detector::*;
pub use image::Image;
pub use structures::{Point, Rect};
pub use extract::ExtremalRegion;
pub use extract::cser::{Incremental, Region};
pub use extract::cser::feature::AspectRatio;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestInc {
    points: Vec<Point>,
    peaks: Vec<(Rect, AspectRatio)>
}

impl Incremental for TestInc {
    fn init(p: Point, _: usize, _: i32) -> Self {
        TestInc { points: vec![p], peaks: vec![] }
    }

    fn increment(&mut self, p: Point, _: i32,  _: &Image<u8>, _: &Image<Option<usize>>) {
        self.points.push(p);
    }

    fn merge(&mut self, r: &TestInc, _: i32, _: &Image<u8>, _: &Image<Option<usize>>) {
        self.points.extend_from_slice(&r.points[..]);
    }
}

impl ExtremalRegion for TestInc {
    type F = AspectRatio;

    fn threshold(&self) -> i32 {
        0
    }

    fn points<'a>(&'a self) -> &'a [Point] {
        &self.points[..]
    }

    fn weight(&self) -> f32 {
        ((self.points.len() % 4) as f32) / 4.0f32
    }

    fn bounds(&self) -> Rect {
        Rect(Point { x: 0, y: 0 }, Point { x: 1, y: 1 })
    }

    fn peaks<'a>(&'a self) -> &'a [(Rect, AspectRatio)] {
        &self.peaks[..]
    }

    fn feature_vec(&self, _: &mut Vec<f32>) {}
}

describe! detect_regions {
    describe! hist {
        it "should return points for each intensity level from 0 to 255" {
            let data = vec![0, 6, 233, 6, 13, 200, 13, 13];
            let image: Image<u8> = Image::from_data(data, 4, 2);
            let hist = hist(&image);

            assert_eq!(hist.len(), 256);
            assert_eq!(hist[0].len(), 1);
            assert_eq!(hist[13].len(), 3);
            assert_eq!(hist[6].len(), 2);
        }
    }

    describe! find_neighbors {
        it "should return indexes of adjacent regions" {
            let b: Vec<u8> = vec![
                0, 1, 2, 0,
                0, 0, 2, 0,
                0, 4, 0, 5,
                0, 0, 6, 7,
            ];

            let mut neighbors_buf: Vec<usize> = vec![];

            let data = b.iter()
                .map(|x| if x.clone() != 0u8 { Some(x.clone() as usize) } else { None })
                .collect();

            let img: Image<Option<usize>> = Image::from_data(data, 4, 4);

            find_neighbors(&img, Point { x: 0, y: 0 }, &mut neighbors_buf);
            assert_eq!(neighbors_buf.len(), 1);

            find_neighbors(&img, Point { x: 2, y: 2 }, &mut neighbors_buf);
            assert_eq!(neighbors_buf.len(), 4);

            find_neighbors(&img, Point { x: 3, y: 3 }, &mut neighbors_buf);
            assert_eq!(neighbors_buf.len(), 2);

            find_neighbors(&img, Point { x: 3, y: 1 }, &mut neighbors_buf);
            assert_eq!(neighbors_buf.len(), 2);

            find_neighbors(&img, Point { x: 3, y: 3 }, &mut neighbors_buf);
            let expected_points = vec![5, 6];
            assert_eq!(neighbors_buf, expected_points);
        }
    }

    describe! process_point {
        before_each {
            let img: Image<u8> = Image::from_data(vec![], 0, 0);

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
                ],
                peaks: vec![]
            };

            let r2 = TestInc {
                points: vec![
                    Point { x: 0, y: 2 },
                    Point { x: 1, y: 2 }
                ],
                peaks: vec![]
            };

            let r3 = TestInc {
                points: vec![
                    Point { x: 3, y: 2 },
                    Point { x: 3, y: 3 }
                ],
                peaks: vec![]
            };

            let mut regions: Vec<TestInc> = vec![r1.clone(), r2.clone(), r3.clone()];
            let mut neighbors_buf: Vec<usize> = vec![];
        }

        it "should create new region and add it to regions list if there are no adjacent regions" {
            let new_point = Point { x: 5, y: 0 };
            let expected_region = TestInc {
                points: vec![new_point],
                peaks: vec![]
            };

            process_point(new_point, 0, &img, &mut reg_img, &mut regions, &mut neighbors_buf);
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

            process_point(Point { x: 5, y: 0 }, 0, &img, &mut reg_img, &mut regions, &mut neighbors_buf);

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

            process_point(Point { x: 0, y: 0 }, 0, &img, &mut reg_img, &mut regions, &mut neighbors_buf);

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

            process_point(Point { x: 1, y: 1 }, 0, &img, &mut reg_img, &mut regions, &mut neighbors_buf);

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

            process_point(Point { x: 2, y: 2 }, 0, &img, &mut reg_img, &mut regions, &mut neighbors_buf);

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
