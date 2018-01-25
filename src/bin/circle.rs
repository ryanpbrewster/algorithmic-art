//! Generate an image of a circle
extern crate image;

use std::fs::File;

fn main() {
    let imgx = 800;
    let imgy = 800;

    let scalex = 4.0 / imgx as f32;
    let scaley = 4.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cy = y as f32 * scaley - 2.0;
        let cx = x as f32 * scalex - 2.0;

        let luminosity = if  cx * cx + cy * cy < 1.0 { 255 } else { 0 };
        // Create an 8bit pixel of type Luma and value i
        // and assign in to the pixel at position (x, y)
        *pixel = image::Luma([luminosity as u8]);

    }


    // Save the image as “fractal.png”
    let ref mut fout = File::create("fractal.png").unwrap();

    // We must indicate the image's color type and what format to save as
    image::ImageLuma8(imgbuf).save(fout, image::PNG).unwrap();
}

