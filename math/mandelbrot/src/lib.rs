#![cfg_attr(not(feature = "std"), no_std)]

use num_traits::Float;

// x is a scaled x coordinate of pixel (scaled to lie in the Mandelbrot X scale (-2.5, 1))
// y is a scaled y coordinate of pixel (scaled to lie in the Mandelbrot Y scale (-1, 1))
pub fn point<T: Float>(cx: T, cy: T) -> Option<usize> {
    let mut zx: T = T::zero();
    let mut zy: T = T::zero();
    let mut count = 0;

    let two = T::one() + T::one();
    let four = two + two;

    loop {
        // sqrt(|zn|) > 2, which means this point belongs to the set.
        if zx * zx + zy * zy > four {
            return Some(count);
        }
        // We ran out of iterations to converge.
        // let upper_bound = upper_bound(count, zx, zy);
        // if count as f32 > upper_bound {
        if count > 1000 {
            return None;
        }

        count += 1;
        let next_x = zx * zx - zy * zy + cx;
        let next_y = two * zx * zy + cy;
        zx = next_x;
        zy = next_y;
    }
}

// fn upper_bound(n: usize, x: f32, y: f32) -> f32 {
//     f32::ln(f32::ln(f32::sqrt(x * x + y * y)) / f32::exp2(n as f32))
// }
