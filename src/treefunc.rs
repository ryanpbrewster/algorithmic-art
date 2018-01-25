use rand;
use std;

#[derive(Debug)]
pub enum PixelFunction {
    X,
    Y,
    Sin(Box<PixelFunction>),
    Cos(Box<PixelFunction>),
    Average(Box<PixelFunction>, Box<PixelFunction>),
    Product(Box<PixelFunction>, Box<PixelFunction>)
}

impl PixelFunction {
    pub fn new<P: rand::Rng>(prng: &mut P, depth: usize) -> PixelFunction {
        if depth == 0 {
            let idx = prng.gen_range(0, 2);
            match idx {
                0 => PixelFunction::X,
                1 => PixelFunction::Y,
                _ => unreachable!()
            }
        } else {
            let idx = prng.gen_range(0, 4);
            match idx {
                0 => PixelFunction::Sin(Box::new(PixelFunction::new(prng, depth - 1))),
                1 => PixelFunction::Cos(Box::new(PixelFunction::new(prng, depth - 1))),
                2 => PixelFunction::Average(
                    Box::new(PixelFunction::new(prng, depth - 1)),
                    Box::new(PixelFunction::new(prng, depth - 1))
                ),
                3 => PixelFunction::Product(
                    Box::new(PixelFunction::new(prng, depth - 1)),
                    Box::new(PixelFunction::new(prng, depth - 1))
                ),
                _ => unreachable!()
            }
        }
    }

    pub fn eval(&self, x: f32, y: f32) -> f32 {
        match *self {
            PixelFunction::X => x,
            PixelFunction::Y => y,
            PixelFunction::Sin(ref f) => (f.eval(x, y) * std::f32::consts::PI).sin(),
            PixelFunction::Cos(ref f) => (f.eval(x, y) * std::f32::consts::PI).cos(),
            PixelFunction::Average(ref f, ref g) => 0.5 * (f.eval(x, y) + g.eval(x, y)),
            PixelFunction::Product(ref f, ref g) => f.eval(x, y) * g.eval(x, y)
        }
    }
}

/// Requires: z in [-1, 1]
///
/// ```
/// use algorithmic_art::treefunc::rescale;
///
/// assert_eq!(rescale(-1.0), 0);
/// assert_eq!(rescale(-0.5), 63);
/// assert_eq!(rescale(0.0), 127);
/// assert_eq!(rescale(0.5), 191);
/// assert_eq!(rescale(1.0), 255);
/// ```
pub fn rescale(z: f32) -> u8 {
    assert!(-1.0 <= z && z <= 1.0, "expected r = {} to be in [-1, 1]", z);
    (127.999 * (z + 1.0)) as u8
}

