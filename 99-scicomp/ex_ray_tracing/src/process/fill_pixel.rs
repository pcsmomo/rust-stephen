//! Utilities for filling all pixels in an image with a specified color.
use image::{Rgb, RgbImage};

/// Fills the entire image with the given color.
///
/// # Arguments
///
/// * `img` - A mutable reference to the image to fill.
/// * `color` - The color to fill the image with.
///
/// # Example
/// ```
/// use image::{Rgb, RgbImage};
/// use crate::process::fill_pixel::fill_image;
/// let mut img = RgbImage::new(100, 100);
/// fill_image(&mut img, Rgb([0, 255, 0]));
/// ```
pub fn fill_image(img: &mut RgbImage, color: Rgb<u8>) {
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            img.put_pixel(x, y, color);
        }
    }
}
