/// A fractal generation module.
pub mod fractal_gen {
    extern crate image;
    extern crate num_complex;

    use self::num_complex::Complex;
    use std;

    /// Complex extension of the Collatz function.
    ///
    /// # Arguments
    /// * `z` - A complex number.
    ///
    /// # Returns
    /// * The Collatz function applied to `z`. Where the Collatz function foll-
    /// ows the definition from here: http://yozh.org/2012/01/12/the_collatz_fractal/
    pub fn collatz(z: Complex<f32>) -> Complex<f32> {
        let comp_pi = Complex::new(std::f32::consts::PI, 0.0);
        return ((7.0 * z + 2.0) 
             - (comp_pi * z).cos() 
             * (5.0 * z + 2.0)) / 4.0;
    }
    
    /// Generate a fractal picture.
    ///
    /// # Arguments
    /// * `buf` - Image buffer that will store the result image.
    /// * `f` - Complex function that returns a complex number.
    /// * `scalex` - Horizontal scaling factor.
    /// * `scaley` - Vertical scaling factor.
    /// * `max_iterations` - Maximum number of applications of `f`.
    pub fn populate_image(buf: &mut image::GrayImage, 
                      f: fn(Complex<f32>) -> Complex<f32>,
                      scalex: f32, scaley: f32,
                      max_iterations: u16) {
        for (x, y, pixel) in buf.enumerate_pixels_mut() {
            let cy = y as f32 * scaley - 2.0;
            let cx = x as f32 * scalex - 2.0;

            let mut z = Complex::new(cx, cy);

            let mut i = 0;

            for t in 0..max_iterations {
                if z.norm() > 1000.0 {
                    break
                }
                z = f(z);
                i = t;
            }
            *pixel = image::Luma([i as u8]);
        }
    }
}
