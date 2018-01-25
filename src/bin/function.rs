//! Make a grayscale image based on the function Exp[-(x^2 + 4y^2)]
extern crate image;

use std::fs::File;

fn main() {
    let imgx = 800;
    let imgy = 800;

    let scalex = 2.0 / imgx as f32;
    let scaley = 2.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cy = y as f32 * scaley - 1.0;
        let cx = x as f32 * scalex - 1.0;

        let value = (-(cx * cx + 4.0 * cy * cy)).exp();
        assert!(0.0 <= value && value <= 1.0, "expected {} to be in [0, 1)", value);
        *pixel = image::Luma([(value * 256.0) as u8]);
    }


    // Save the image as “circle.png”
    let ref mut fout = File::create(format!("{}.png", module_path!())).unwrap();

    // We must indicate the image's color type and what format to save as
    image::ImageLuma8(imgbuf).save(fout, image::PNG).unwrap();
}
