use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::io::{self, Write};
use structopt::StructOpt;

extern crate image_preview;

#[derive(StructOpt)]
struct Cli {
    filename: String,
    #[structopt(
        long,
        help = "When supplied, renders the image in true colour (not supported on all terminals). The default is to use the nearest colour from the 255 indexed colours of the standard ANSI palette."
    )]
    true_color: bool,
    #[structopt(
        long,
        help = "Resize filter used when scaling for the terminal (allowed values: nearest, triangle, catmullrom, gaussian, lanczos3)",
        default_value = "lanczos3"
    )]
    filter_type: String,
    #[structopt(long, short, help = "When supplied, limits the image with to the number of columns, else probes the terminal to determine the displayable width.")]
    width: Option<u32>,
}

fn main() {
    let args = Cli::from_args();
    let img = ImageReader::open(args.filename)
        .expect("File does not exist")
        .decode()
        .expect("Cannot decode image");
    let filter_type = image_preview::from_str(&args.filter_type).expect("Unsupported filter type");
    let (width, height) = image_preview::calc_size(&img, args.width).unwrap();
    let resized_img = img.resize(width, height, filter_type);

    let pixel_renderer = if args.true_color {
        image_preview::true_color
    } else {
        image_preview::indexed
    };
    let mut buf = Vec::new();
    for y in (0..resized_img.height() - 1).step_by(2) {
        for x in 0..resized_img.width() {
            buf.write(pixel_renderer(&resized_img, x, y).as_bytes())
                .unwrap();
        }
        buf.write(b"\x1b[m\n").unwrap();
    }
    io::stdout().write(&buf).unwrap();
}
