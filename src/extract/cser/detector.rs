use image::Image;
use structures::Point;
use super::Incremental;
pub use super::region::Region;

pub fn detect_regions<T: Incremental> (image: &Image<u8>) -> Vec<T> {
    let baskets = hist(image);
    let mut all_regions: Vec<Region> = vec![];
    let mut reg_image: Image<Option<usize>> = image.map( |_| None );

    for i in 0..255 {
        let points = &baskets[i];
        for p in points {
            process_point(p.clone(), &mut reg_image, &mut all_regions);
        }
    }

    vec![]
}

fn process_point(p: Point, reg_image: &mut Image<Option<usize>>, all_regions: &mut Vec<Region>) {
    let ns = find_neighbors(&reg_image, p.clone());

    if ns.len() == 0 {
        let n = ns[0].clone();
        all_regions.push(Incremental::init(n));
        let idx = all_regions.len() - 1;
        reg_image.set_pixel(n.x, n.y, Some(idx));
    }
    // if there
    else if ns.len() == 1 {
    }
    else if ns.len() == 2 {

    }
    else {

    }
}

fn hist(image: &Image<u8>) -> Vec<Vec<Point>> {
    let mut baskets: Vec<Vec<Point>> = vec![];

    for _ in 0..255 {
        baskets.push(vec![])
    }

    for x in 0..image.width() {
        for y in 0..image.height() {
            let intensity = image[(x, y)];
            baskets[intensity as usize].push(Point { x: (x as i32), y: (y as i32) });
        }
    }

    return baskets;
}

fn find_neighbors(reg_image: &Image<Option<usize>>, p: Point) -> Vec<Point> {
    let neighbors = vec![
        Point { x: -1, y: 0 }, Point { x: 1, y: 0 },
        Point { x: 0, y: -1 }, Point { x: 0, y: 1 }
    ];

    let points = neighbors.iter()
        .map(|n| Point { x: p.x - n.x, y: p.y - n.y });

    let inside_points = points.filter(|p| reg_image.inside(p.x, p.y));
    let some_points = inside_points.filter(|p| reg_image[(p.x as usize, p.y as usize)].is_some());

    some_points.collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use super::{hist, find_neighbors};
    use image::Image;
    use structures::Point;
    use extract::cser::Incremental;

    #[test]
    fn hist_test() {
        let data = vec![0, 6, 233, 6, 13, 200, 13, 13];
        let image: Image<u8> = Image::from_data(data, 4, 2);
        let hist = hist(&image);

        assert_eq!(hist.len(), 255);
        assert_eq!(hist[0].len(), 1);
        assert_eq!(hist[13].len(), 3);
        assert_eq!(hist[6].len(), 2);
    }

    #[test]
    fn count_neighbors_test() {
        let r: Region = Incremental::init(Point { x: 0, y: 0 });
        let b: Vec<u8> = vec![
            0, 1, 1, 0,
            0, 0, 1, 0,
            0, 1, 0, 1,
            0, 0, 1, 1,
        ];

        let data = b.iter()
            .map(|x| if x.clone() == 1u8 { Some(1usize) } else { None })
            .collect();

        let img: Image<Option<usize>> = Image::from_data(data, 4, 4);
        let reg: Vec<Region> = vec![];

        assert_eq!(find_neighbors(&img, Point { x: 0, y: 0 }).len(), 1);
        assert_eq!(find_neighbors(&img, Point { x: 2, y: 2 }).len(), 4);
        assert_eq!(find_neighbors(&img, Point { x: 3, y: 3 }).len(), 2);
        assert_eq!(find_neighbors(&img, Point { x: 3, y: 1 }).len(), 2);

        let expected_points = vec![
            Point { x: 2, y: 3 },
            Point { x: 3, y: 2 }
        ];
        assert_eq!(find_neighbors(&img, Point { x: 3, y: 3 }), expected_points);
    }
}
