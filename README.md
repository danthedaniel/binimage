binimage
===

Create an image from the binary data of a file.

### Building

Build with cargo: `cargo build --release`

```
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
```

### Examples

Running `binimage` on an Atari ROM produces interesting results. I used the
parameters `--width=8 --bitdepth=1` here.

![binimage ran on MarioBros](examples/mario_bros.png)

This is the image produced when `binimage` is ran on its own binary.

![binimage ran on itself](examples/binimage.png)
