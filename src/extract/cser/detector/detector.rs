use image::Image;
use structures::Point;
use extract::cser::Incremental;

pub fn detect_regions<T: Incremental + Sized> (image: &Image<u8>) -> Vec<T> {
    let baskets = hist(image);
    let mut all_regions: Vec<T> = vec![];
    let mut reg_image: Image<Option<usize>> = image.map( |_| None );

    for i in 0..255 {
        let points = &baskets[i];
        for p in points {
            process_point(p.clone(), &mut reg_image, &mut all_regions);
        }
    }

    return all_regions;
}

pub fn process_point<T: Incremental + Sized>(
    p: Point,
    reg_image: &mut Image<Option<usize>>,
    all_regions: &mut Vec<T>) {

    let ns = find_neighbors(&reg_image, p.clone());

    match &ns[..] {
        [] => {
            all_regions.push(Incremental::init(p));
            let idx = all_regions.len() - 1;
            reg_image.set_pixel(p.x, p.y, Some(idx));
        },
        [r_idx] => {
            let r = &mut (all_regions[r_idx]);
            r.increment(p);
            reg_image.set_pixel(p.x, p.y, Some(r_idx));
        },
        [r1_idx, r2_idx] => {
            if let Some((r1, r2)) = index_twice(&mut all_regions[..], r1_idx, r2_idx) {
                r1.increment(p);
                reg_image.set_pixel(p.x, p.y, Some(r1_idx));

                r1.merge(r2);
                if let Some(points) = r2.points() {
                    for p in points {
                        reg_image.set_pixel(p.x, p.y, Some(r1_idx));
                    }
                }
            } else {
                panic!("failed to get regions - got None");
            }
        },
        _ => {
            panic!("more than 2 neighbors, don't know how to handle it!");
        }
    }
}

pub fn hist(image: &Image<u8>) -> Vec<Vec<Point>> {
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

pub fn find_neighbors(reg_image: &Image<Option<usize>>, p: Point) -> Vec<usize> {
    let neighbors = vec![
        Point { x: -1, y: 0 }, Point { x: 1, y: 0 },
        Point { x: 0, y: -1 }, Point { x: 0, y: 1 }
    ];

    let points = neighbors.iter()
        .map(|n| Point { x: p.x - n.x, y: p.y - n.y });

    let inside_points = points.filter(|p| reg_image.inside(p.x, p.y));
    let some_points = inside_points.filter(|p| reg_image[(p.x as usize, p.y as usize)].is_some());
    let mut indexes: Vec<usize> = some_points
        .map(|p| reg_image[(p.x as usize, p.y as usize)].unwrap())
        .collect();

    indexes.sort();
    indexes.dedup();

    return indexes;
}

pub fn index_twice<T>(slc: &mut [T], a: usize, b: usize) -> Option<(&mut T, &mut T)> {
    if a >= slc.len() || b >= slc.len() {
        None
    } else {
        unsafe {
            let ar = &mut *(slc.get_unchecked_mut(a) as *mut _);
            let br = &mut *(slc.get_unchecked_mut(b) as *mut _);
            Some((ar, br))
        }
    }
}
