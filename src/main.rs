extern crate image;

use std::env;
use std::error::Error;
use std::io::Read;
use std::fs::File;
use std::path::Path;

fn help() {
    println!("Usage: binimage <bin_file> <output_image>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            render_file(&args[1], &args[2]);
        },
        _ => {
            help();
        }
    }
}

// The shape to give an image from its file size
fn image_shape(buffer_size: usize) -> (u32, u32) {
    let num_pixels = (buffer_size as f32) / 3.0;
    let width = num_pixels.sqrt() as u32;
    let height = (num_pixels / (width as f32)).ceil() as u32;

    return (width, height);
}

// The number of additional bytes necessary to match the buffer size and image size (in pixels)
fn bytes_to_add(buffer_size: usize, dims: (u32, u32)) -> u32 {
    let bytes_required = dims.0 * dims.1 * 3; // 3 bytes per pixel
    return bytes_required - (buffer_size as u32);
}

fn render_file(bin_file: &String, image_file: &String) {
    let input_path = Path::new(bin_file);
    let display = input_path.display();

    // Read in binary file
    let mut file = match File::open(input_path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't open {}: {}", display, why.description())
    };

    let mut buffer: Vec<u8> = Vec::new();
    let size = match file.read_to_end(&mut buffer) {
        Ok(size) => size,
        Err(why) => panic!("Couldn't read file {}: {}", display, why)
    };

    let dims = image_shape(size);
    let size_diff = bytes_to_add(size, dims);

    // Add any extra bytes onto the end as black pixels
    for _ in 0..size_diff {
        buffer.push(0);
    }

    println!("Size difference: {}", size_diff);
    println!("Buffer length: {}\n# of pixels: {}", size, (dims.0 * dims.1));

    // Write image
    image::save_buffer(&Path::new(image_file), &buffer, dims.0, dims.1, image::RGB(8)).unwrap();
}
