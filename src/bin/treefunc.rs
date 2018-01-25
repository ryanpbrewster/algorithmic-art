//! Make an RGB image based on a recursive tree of functions

extern crate structopt;

#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

extern crate image;

extern crate rand;
use rand::SeedableRng;

use std::fs::File;
use std::hash::Hasher;

extern crate algorithmic_art;
use algorithmic_art::treefunc;
use algorithmic_art::treefunc::PixelFunction;

#[derive(StructOpt, Debug)]
#[structopt(name = "treefunc", about = "Use a recurisve tree of simple functions to make RGB images")]
struct CliOptions {
    #[structopt(short = "d", long = "depth", help = "How tall to make the function tree", default_value = "8")]
    depth: usize,
    #[structopt(short = "o", long = "output", help = "filepath for generated image", default_value = "treefunc.png")]
    output: String,
    #[structopt(short = "s", long = "seed", help = "Seed the PRNG", default_value = "<default_seed>")]
    seed: String,
    #[structopt(short = "v", long = "verbose", help = "Print debugging info")]
    verbose: bool,
}

fn main() {
    let opts: CliOptions = CliOptions::from_args();
    let imgx = 800;
    let imgy = 800;

    let scalex = 2.0 / imgx as f32;
    let scaley = 2.0 / imgy as f32;

    let mut hsh = std::collections::hash_map::DefaultHasher::new();
    hsh.write(opts.seed.as_bytes());
    let seed = hsh.finish();
    let a = (seed >> 0) as u32;
    let b = (seed >> 32) as u32;
    if opts.verbose {
        println!("seed = {} --> {}, {}", seed, a, b);
    }
    let mut prng = rand::XorShiftRng::from_seed([a, b, 0, 0]);
    let rfn = PixelFunction::new(&mut prng, opts.depth);
    let gfn = PixelFunction::new(&mut prng, opts.depth);
    let bfn = PixelFunction::new(&mut prng, opts.depth);

    if opts.verbose {
        println!("r = {:?}", rfn);
        println!("g = {:?}", gfn);
        println!("b = {:?}", bfn);
    }

    // Create a new ImgBuf with width: imgx and height: imgy
    let imgbuf = image::ImageBuffer::from_fn(imgx, imgy, |px, py| {
        let x = px as f32 * scalex - 1.0;
        let y = py as f32 * scaley - 1.0;

        let r = treefunc::rescale(rfn.eval(x, y));
        let g = treefunc::rescale(gfn.eval(x, y));
        let b = treefunc::rescale(bfn.eval(x, y));
        image::Rgb([r, g, b])
    });

    let ref mut fout = File::create(opts.output).unwrap();

    // We must indicate the image's color type and what format to save as
    image::ImageRgb8(imgbuf).save(fout, image::PNG).unwrap();
}
