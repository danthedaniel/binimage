#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate image;

use std::io::Read;
use std::fs::File;
use std::path::Path;

use docopt::Docopt;

const BITS_PER_BYTE: f32 = 8.0;
const USAGE: &'static str = "
binimage
Create an image from the binary data of a file.

Usage:
  binimage <input> <output> [--width=<pixels>] [--bitdepth=<bits>]
  binimage <input> <output> [--height=<pixels>] [--bitdepth=<bits>]
  binimage (-h | --help)

Options:
  -h --help          Show this screen.
  --width=<pixels>   Specify output image width. Default is sqrt of the file size.
  --height=<pixels>  Specify output image height. Default is sqrt of the file size.
  --bitdepth=<bits>  Number of bits per pixel. Default is 24. Less than 12 is grayscale.
                     Valid values: 1, 2, 4, 8, 12, 24
";

#[derive(Copy, PartialEq, Eq, Debug, Clone)]
enum ColorType {
    Gray(u8),
    RGB(u8)
}

impl ColorType {
    fn to_image_colortype(&self) -> image::ColorType {
        match self {
            &ColorType::Gray(n) => image::Gray(n),
            &ColorType::RGB(n) => image::RGB(n)
        }
    }
}

#[derive(Debug, Deserialize)]
struct Args {
    arg_input: String,
    arg_output: String,
    flag_width: u32,
    flag_height: u32,
    flag_bitdepth: u8
}

fn main() {
    let args: Args = Docopt::new(USAGE).
        and_then(|d| d.deserialize()).
        unwrap_or_else(|e| e.exit());

    match render_file(args) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1)
        }
    }
}

fn colortype_from_bitdepth(bitdepth: u8) -> Result<ColorType, &'static str> {
    match bitdepth {
        0  => Ok(ColorType::RGB(8)),
        1  => Ok(ColorType::Gray(1)),
        2  => Ok(ColorType::Gray(2)),
        4  => Ok(ColorType::Gray(4)),
        8  => Ok(ColorType::Gray(8)),
        12 => Ok(ColorType::RGB(4)),
        24 => Ok(ColorType::RGB(8)),
        _  => Err("Unsupported bitdepth")
    }
}

fn bits_per_pixel(c: ColorType) -> u32 {
    match c {
        ColorType::Gray(n) => n as u32,
        ColorType::RGB(n) => 3 * n as u32,
    }
}

// The shape to give an image from its file size
fn image_shape(buffer_size: usize, arg_size: (u32, u32), colortype: ColorType) -> Result<(u32, u32), &'static str> {
    let bit_depth = bits_per_pixel(colortype);
    let num_pixels = ((buffer_size as f32) * BITS_PER_BYTE / bit_depth as f32).ceil();

    if arg_size.0 > num_pixels as u32 || arg_size.1 > num_pixels as u32 {
        return Err("Height or width is too large.");
    }

    match arg_size {
        (0, 0) => {
            let width = num_pixels.sqrt() as u32;
            let height = (num_pixels / (width as f32)).ceil() as u32;
            Ok((width, height))
        },
        (x, 0) => {
            let height = (num_pixels / (x as f32)).ceil() as u32;
            Ok((x, height))
        },
        (0, y) => {
            let width = (num_pixels / (y as f32)).ceil() as u32;
            Ok((width, y))
        },
        _ => Err("Height and width can not both be provided.")
    }
}

// The number of additional bytes necessary to match the buffer size and image size (in pixels)
fn bytes_to_add(buffer_size: usize, dims: (u32, u32), colortype: ColorType) -> Result<u32, &'static str> {
    let bit_depth = bits_per_pixel(colortype);
    let bits_required = dims.0 * dims.1 * bit_depth;
    let bytes_required = (bits_required as f32 / BITS_PER_BYTE).ceil() as u32;

    Ok(bytes_required - buffer_size as u32)
}

fn render_file(args: Args) -> Result<(), &'static str> {
    let input_path = Path::new(&args.arg_input);
    let output_path = Path::new(&args.arg_output);

    // Read in binary file
    let mut file = File::open(input_path).
        map_err(|_| "Couldn't open input file.")?;

    let mut buffer: Vec<u8> = Vec::new();
    let file_size = file.read_to_end(&mut buffer).
        map_err(|_| "Couldn't read input file.")?;

    let colortype = colortype_from_bitdepth(args.flag_bitdepth)?;
    let arg_size = (args.flag_width, args.flag_height);
    let dims = image_shape(file_size, arg_size, colortype)?;
    let size_diff = bytes_to_add(file_size, dims, colortype)?;

    // Add any extra bytes onto the end as black pixels
    for _ in 0..size_diff {
        buffer.push(0);
    }

    // Write image
    image::save_buffer(&output_path, &buffer, dims.0, dims.1, colortype.to_image_colortype()).unwrap();

    Ok(())
}
