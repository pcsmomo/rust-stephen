mod process;

use image::{Rgb, RgbImage};

use process::fill_colors::fill_gradient;

fn fill_image_with_gradient() {
    // constants
    const RESULT_DIR: &str = "results";
    const RESULT_FILE_NAME: &str = "image.png";
    const IMAGE_WIDTH: u32 = 600;
    const IMAGE_HEIGHT: u32 = 600;

    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    fill_gradient(&mut img);

    img.save(format!("{}/{}", RESULT_DIR, RESULT_FILE_NAME))
        .unwrap();
}

fn main() {
    fill_image_with_gradient();
}
