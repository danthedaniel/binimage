binimage
===

Create an image from the binary data of a file.

### Building

Build with cargo: `cargo build --release`

```
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
```

### Examples

Running `binimage` on an Atari ROM produces interesting results. I used the
parameters `--width=8 --bitdepth=1` here.

![binimage ran on MarioBros](examples/mario_bros.png)

This is the image produced when `binimage` is ran on its own binary.

![binimage ran on itself](examples/binimage.png)
