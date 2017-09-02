binimage
===

Create an image from the binary data of a file.

### Building

Build with cargo: `cargo build --release`

```
Usage:
  binimage <input> <output> [--width=<pixels>]
  binimage <input> <output> [--height=<pixels>]
  binimage (-h | --help)

Options:
  -h --help         Show this screen.
  --width=<pixels>  Specify output image width (default is sqrt of the file size).
  --height=<pixels>
```

### Example

This is the image produced when `binimage` is ran on its own binary.

![binimage ran on itself](example.png)
