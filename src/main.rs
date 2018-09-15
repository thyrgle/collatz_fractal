extern crate fraclib;
extern crate argparse;
extern crate image;

use argparse::{ArgumentParser, Store};
use fraclib::fractal_gen;

/// Generates and exports a fractal.png. fractal.png is a picture of the Colla-
/// tz fractal.
///
/// # Flags
///
/// * `--maxiter` iteration depth for successive iteration on the Collatz func-
/// tion.
/// * `--xdim` width of the image.
/// * `--ydim` height of the image.
/// * `--scalewidth` horizontal scaling factor.
/// * `--scaleheight` vertical scaling factor.
fn main() {
    let mut max_iterations = 1024u16;

    let mut imgx = 800;
    let mut imgy = 800;

    let mut scalex = 4.0 / imgx as f32;
    let mut scaley = 4.0 / imgy as f32;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Create a Collatz Fractal.");
        ap.refer(&mut max_iterations)
            .add_option(&["-i", "--maxiter"], Store,
            "Maximum number of iterations on Collatz function");
        ap.refer(&mut imgx)
            .add_option(&["-x", "--xdim"], Store,
            "The x dimension of the generated image");
        ap.refer(&mut imgy)
            .add_option(&["-y", "--ydim"], Store,
            "The y dimension of the generated image");
        ap.refer(&mut scalex)
            .add_option(&["-w", "--scalewidth"], Store,
            "The width scaling factor");
        ap.refer(&mut scaley)
            .add_option(&["-h", "--scaleheight"], Store,
            "The height scaling factor");
        ap.parse_args_or_exit();
    }

    let mut imgbuf = image::GrayImage::new(imgx, imgy);

    fractal_gen::populate_image(&mut imgbuf, 
                                fractal_gen::collatz,
                                scalex, scaley,
                                max_iterations);

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
}
