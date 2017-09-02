#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate image;

use std::error::Error;
use std::io::Read;
use std::fs::File;
use std::path::Path;

use docopt::Docopt;

const USAGE: &'static str = "
binimage
Create an image from the binary data of a file.

Usage:
  binimage <input> <output> [--width=<pixels>]
  binimage <input> <output> [--height=<pixels>]
  binimage (-h | --help)

Options:
  -h --help         Show this screen.
  --width=<pixels>  Specify output image width (default is sqrt of the file size).
  --height=<pixels>
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_input: String,
    arg_output: String,
    flag_width: u32,
    flag_height: u32
}

fn main() {
    let args: Args = Docopt::new(USAGE).
                        and_then(|d| d.deserialize()).
                        unwrap_or_else(|e| e.exit());

    render_file(args);
}

// The shape to give an image from its file size
fn image_shape(buffer_size: usize, arg_width: u32, arg_height: u32) -> (u32, u32) {
    let arg_size = (arg_width, arg_height);
    let num_pixels = (buffer_size as f32) / 3.0;

    if arg_size.0 > num_pixels as u32 || arg_size.1 > num_pixels as u32 {
        panic!("Neither height nor width can be greater than the file size / 3.");
    }

    match arg_size {
        (0, 0) => {
            let width = num_pixels.sqrt() as u32;
            let height = (num_pixels / (width as f32)).ceil() as u32;
            return (width, height);
        },
        (x, 0) => {
            let height = (num_pixels / (x as f32)).ceil() as u32;
            return (x, height);
        },
        (0, y) => {
            let width = (num_pixels / (y as f32)).ceil() as u32;
            return (width, y);
        },
        _ => panic!("Height and width can not both be provided.")
    }
}

// The number of additional bytes necessary to match the buffer size and image size (in pixels)
fn bytes_to_add(buffer_size: usize, dims: (u32, u32)) -> u32 {
    let bytes_required = dims.0 * dims.1 * 3; // 3 bytes per pixel
    return bytes_required - (buffer_size as u32);
}

fn render_file(args: Args) {
    let input_path = Path::new(&args.arg_input);
    let output_path = Path::new(&args.arg_output);

    // Read in binary file
    let mut file = match File::open(input_path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't open {}: {}", input_path.display(), why.description())
    };

    let mut buffer: Vec<u8> = Vec::new();
    let size = match file.read_to_end(&mut buffer) {
        Ok(size) => size,
        Err(why) => panic!("Couldn't read {}: {}", input_path.display(), why.description())
    };

    let dims = image_shape(size, args.flag_width, args.flag_height);
    let size_diff = bytes_to_add(size, dims);

    // Add any extra bytes onto the end as black pixels
    for _ in 0..size_diff {
        buffer.push(0);
    }

    // Write image
    image::save_buffer(&output_path, &buffer, dims.0, dims.1, image::RGB(8)).unwrap();
}
