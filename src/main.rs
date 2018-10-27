#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate image;

mod color;

use std::io::Read;
use std::fs::File;

use docopt::Docopt;
use color::ColorType;

const USAGE: &'static str = "
binimage
Create an image from the binary data of a file.

Usage:
  binimage <input> [output] [--width=<pixels>] [--bitdepth=<bits>]
  binimage <input> [output] [--height=<pixels>] [--bitdepth=<bits>]
  binimage (-h | --help)

Options:
  -h --help          Show this screen
  output             Default is out.png
  --width=<pixels>   Specify output image width.
  --height=<pixels>  Specify output image height.
  --bitdepth=<bits>  Number of bits per pixel. Default is 24. Less is grayscale
                     Valid values: 1, 2, 4, 8, 24
";

#[derive(Deserialize)]
struct Args {
    arg_input: String,
    arg_output: Option<String>,
    flag_width: Option<u32>,
    flag_height: Option<u32>,
    flag_bitdepth: Option<u8>
}

fn main() {
    let args: Args = Docopt::new(USAGE).
        and_then(|d| d.deserialize()).
        unwrap_or_else(|e| e.exit());

    match render_file(args) {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1)
        }
    }
}

/// Round up an integer division.
///
/// `numerator` - The upper component of a division
/// `denominator` - The lower component of a division
fn int_ceil(numerator: u32, denominator: u32) -> u32 {
    return (numerator + denominator - 1) / denominator;
}

/// Determine the dimensions to give to the generated image.
///
/// `buffer_size` - The file size
/// `arg_shape` - The shape specified by the command line arguments
/// `colortype` - The type of pixel to use
fn image_shape(
    buffer_size: usize,
    arg_shape:   (Option<u32>, Option<u32>),
    colortype:   ColorType
) -> Result<(u32, u32), &'static str> {
    let num_pixels = (buffer_size as f32 / colortype.bytes_per_pixel()).ceil() as u32;

    if arg_shape.0.unwrap_or(0) > num_pixels { Err("Width is too large.")? }
    if arg_shape.1.unwrap_or(0) > num_pixels { Err("Height is too large.")? }

    match arg_shape {
        (None, None) => {
            let width = (num_pixels as f32).sqrt() as u32;
            let height = int_ceil(num_pixels, width);
            Ok((width, height))
        },
        (Some(x), None) => {
            let height = int_ceil(num_pixels, x);
            Ok((x, height))
        },
        (None, Some(y)) => {
            let width = int_ceil(num_pixels, y);
            Ok((width, y))
        },
        _ => unreachable!()
    }
}

/// The number of additional bytes necessary to match the buffer size and image
/// size (in pixels).
///
/// `buffer_size` - The file size
/// `arg_shape` - The shape of the output image
/// `colortype` - The type of pixel to use
fn bytes_to_add(
    buffer_size: usize,
    dims:        (u32, u32),
    colortype:   ColorType
) -> u32 {
    let bit_depth = colortype.bits_per_pixel();
    let bits_required = dims.0 * dims.1 * bit_depth;
    // Round up a byte if necessary
    let bytes_required = int_ceil(bits_required, 8);

    return bytes_required - buffer_size as u32;
}

/// Given a set of CLI arguments, generate an image file from an input file.
///
/// `args` - The argument struct
fn render_file(args: Args) -> Result<(), &'static str> {
    // Read in binary file
    let mut file = File::open(&args.arg_input).
        map_err(|_| "Couldn't open input file.")?;

    let mut buffer: Vec<u8> = Vec::new();
    let file_size = file.read_to_end(&mut buffer).
        map_err(|_| "Couldn't read input file.")?;

    let colortype = ColorType::from_bitdepth(args.flag_bitdepth.unwrap_or(24))?;
    let arg_shape = (args.flag_width, args.flag_height);
    let dims = image_shape(file_size, arg_shape, colortype)?;
    let size_diff = bytes_to_add(file_size, dims, colortype);

    // Add any extra bytes onto the end as black pixels
    for _ in 0..size_diff {
        buffer.push(0);
    }

    // Write image
    image::save_buffer(
        &args.arg_output.unwrap_or("out.png".to_string()),
        &buffer,
        dims.0,
        dims.1,
        colortype.to_image_colortype()
    ).unwrap();

    Ok(())
}
