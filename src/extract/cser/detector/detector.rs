use image::Image;
use structures::Point;
use extract::cser::{Incremental, ExtremalRegion, Trace, Feature};

pub fn detect_regions<
    F: Feature, A: Incremental + ExtremalRegion<F> + Sized,
    B: Trace<F, A>
> (image: &Image<u8>, trace: &B) -> Vec<A> {
    let baskets = hist(image);
    let mut all_regions: Vec<A> = vec![];
    let mut reg_image: Image<Option<usize>> = image.map( |_| None );

    let mut neighbors_buf: Vec<usize> = vec![];

    for i in 0..255 {
        let points = &baskets[i];
        for p in points {
            process_point(p.clone(), image, &mut reg_image, &mut all_regions, &mut neighbors_buf);
        }
        trace.step(i as i32, &all_regions, &reg_image);
    }

    trace.result(&all_regions, &reg_image);

    return all_regions;
}

pub fn process_point<F: Feature,  A: Incremental + ExtremalRegion<F> + Sized>(
    p: Point,
    img: &Image<u8>,
    reg_image: &mut Image<Option<usize>>,
    all_regions: &mut Vec<A>,
    neighbors_buf: &mut Vec<usize>
) {
    find_neighbors(&reg_image, p.clone(), neighbors_buf);

    match &mut neighbors_buf[..] {
        [] => {
            let idx = all_regions.len();
            all_regions.push(A::init(p, idx));
            reg_image.set_pixel(p.x, p.y, Some(idx));
        },
        [r_idx] => {
            let r = &mut (all_regions[r_idx]);
            r.increment(p, img, reg_image);
            reg_image.set_pixel(p.x, p.y, Some(r_idx));
        },
        [all..] => {
            all.sort_by(|a, b| all_regions[*a].points().len().cmp(&all_regions[*b].points().len()));
            all.reverse();
            match all {
                [r1_idx, rest..] => {
                    all_regions[r1_idx].increment(p, img, reg_image);
                    reg_image.set_pixel(p.x, p.y, Some(r1_idx));
                    for r_idx in rest {
                        if let Some((r1, r2)) = index_twice(&mut all_regions[..], r1_idx, *r_idx) {
                            r1.merge(r2, img, reg_image);
                            for p in r2.points() {
                                reg_image.set_pixel(p.x, p.y, Some(r1_idx));
                            }
                        } else {
                            panic!("failed to index regions");
                        }
                    }
                },
                _ => panic!("can't happen")
            }
        }
    }
}

pub fn find_neighbors(reg_image: &Image<Option<usize>>, p: Point, res: &mut Vec<usize>) {
    let neighbors = [
        (p.x - 1, p.y), (p.x, p.y - 1),
        (p.x + 1, p.y), (p.x, p.y + 1)
    ];

    res.clear();
    for n in neighbors.iter() {
        let (x, y) = *n;
        if reg_image.inside(x, y) {
            let reg_option = reg_image[(x as usize, y as usize)];
            if let Some(reg_idx) = reg_option {
                res.push(reg_idx);
            }
        }
    }

    res.sort();
    res.dedup();
}

pub fn hist(image: &Image<u8>) -> Vec<Vec<Point>> {
    let mut baskets: Vec<Vec<Point>> = vec![];

    for _ in 0..256 {
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

pub fn index_twice<T>(slc: &mut [T], a: usize, b: usize) -> Option<(&mut T, &mut T)> {
    debug_assert!(a != b);
    debug_assert!(a < slc.len() && b < slc.len());
    unsafe {
        let ar = &mut *(slc.get_unchecked_mut(a) as *mut _);
        let br = &mut *(slc.get_unchecked_mut(b) as *mut _);
        Some((ar, br))
    }
}
