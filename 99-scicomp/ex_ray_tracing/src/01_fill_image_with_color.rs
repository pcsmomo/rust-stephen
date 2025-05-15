mod process;

use image::{Rgb, RgbImage};

use process::fill_pixel::fill_image;

fn fill_image_with_color() {
    // constants
    const RESULT_DIR: &str = "results";
    const RESULT_FILE_NAME: &str = "image.png";
    const IMAGE_WIDTH: u32 = 100;
    const IMAGE_HEIGHT: u32 = 100;

    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let green = Rgb([0, 255, 0]);
    // let red = Rgb([255, 0, 0]);

    fill_image(&mut img, green);

    img.save(format!("{}/{}", RESULT_DIR, RESULT_FILE_NAME))
        .unwrap();
}

fn main() {
    fill_image_with_color();
}
