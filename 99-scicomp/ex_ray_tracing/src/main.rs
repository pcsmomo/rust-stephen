use image::{Rgb, RgbImage};

const RESULT_DIR: &str = "results";
const RESULT_FILE_NAME: &str = "image.png";

fn main() {
    let mut img = RgbImage::new(100, 100);

    let green = Rgb([0, 255, 0]);

    for y in 0..100 {
        for x in 0..100 {
            img.put_pixel(x, y, green);
        }
    }

    img.save(format!("{}/{}", RESULT_DIR, RESULT_FILE_NAME))
        .unwrap();
}
