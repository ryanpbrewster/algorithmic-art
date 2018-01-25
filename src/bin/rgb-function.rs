//! Make an RGB image based on the function Sin[x^2 + y^2]
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

        let v = cx * cx + cy * cy;
        let r = (v.sin() + 1.0) / 2.0;
        let g =( v.cos() + 1.0) / 2.0;
        let b = (v.atan() * std::f32::consts::FRAC_2_PI + 1.0) / 2.0;
        assert!(0.0 <= r && r <= 1.0, "expected r = {} to be in [0, 1)", r);
        assert!(0.0 <= g && g <= 1.0, "expected g = {} to be in [0, 1)", g);
        assert!(0.0 <= b && b <= 1.0, "expected b = {} to be in [0, 1)", b);
        *pixel = image::Rgb([(256.0 * r) as u8, (256.0 * g) as u8, (256.0 * b) as u8]);
    }


    let ref mut fout = File::create(format!("{}.png", module_path!())).unwrap();

    // We must indicate the image's color type and what format to save as
    image::ImageRgb8(imgbuf).save(fout, image::PNG).unwrap();
}
