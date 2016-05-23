pub use image::Image;

describe! image {
    before_each {
        let data = vec![
            1, 2, 3,
            4, 5, 6
        ];
    }

    describe! from_data {
        it "should create image of specified dimensions" {
            let img: Image<u8> = Image::from_data(data, 3, 2);
            assert_eq!(img.dimensions(), (3, 2));
        }

        failing "if data size does not match dimensions" {
            let _: Image<u8> = Image::from_data(data, 3, 4);
        }
    }

    describe! get_pixel {
        before_each {
            let img = Image::from_data(data, 3, 2);
        }

        it "should get pixel value by coords" {
            assert_eq!(img[(2, 1)], 6);
            assert_eq!(img[(0, 0)], 1);
            assert_eq!(img[(1, 1)], 5);
            assert_eq!(img[(2, 0)], 3);
        }

        failing "if pixel coord is out of bounds" {
            img[(2, 2)];
        }
    }

    describe! inside {
        before_each {
            let img = Image::from_data(data, 3, 2);
        }

        it "should return true if point is in bounds" {
            assert_eq!(img.inside(2, 1), true);
            assert_eq!(img.inside(0, 0), true);
            assert_eq!(img.inside(2, 0), true);
        }

        it "should return false if point is out of bounds" {
            assert_eq!(img.inside(-1, 0), false);
            assert_eq!(img.inside(3, 2), false);
        }
    }
}
