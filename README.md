# Image Preview

A first foray into **rust-lang**. Once build, the program takes in an image file, and
using ANSI escape sequences, outputs a low-fidelity preview of the image in the terminal.
Inspired in part by the **asciiblock** emulator [here](https://github.com/rm-hull/luma.emulator).

![screenshot](https://raw.githubusercontent.com/rm-hull/image-preview/master/screenshot.png)

```console
$ target/release/image-preview -h
image-preview 0.1.0

USAGE:
    image-preview [FLAGS] [OPTIONS] <filename>

FLAGS:
    -h, --help          Prints help information
        --true-color    When supplied, renders the image in true colour (not supported on all terminals). The default is
                        to use the nearest colour from the 255 indexed colours of the standard ANSI palette.
    -V, --version       Prints version information

OPTIONS:
        --filter-type <filter-type>    Resize filter used when scaling for the terminal (allowed values: nearest,
                                       triangle, catmullrom, gaussian, lanczos3) [default: lanczos3]
    -w, --width <width>                When supplied, limits the image with to the number of columns, else probes the
                                       terminal to determine the displayable width.

ARGS:
    <filename>
```
