use cached::proc_macro::cached;
use image::imageops::FilterType;
use image::GenericImageView;
use image::Pixel;
use terminal_size::{terminal_size, Width};

// From a comment by @TerrorBite on https://gist.github.com/MicahElliott/719710
// Generate a list of midpoints of the colorcube
#[cached]
fn snaps() -> Vec<u8> {
    // Default color levels for the color cube
    vec![0x00, 0x5f, 0x87, 0xaf, 0xd7, 0xff]
        .windows(2)
        .map(|pair| (((pair[0] as u16 + pair[1] as u16) / 2) as u8))
        .collect()
}

#[cached(size = 500)]
pub fn rgb2short(r: u8, g: u8, b: u8) -> usize {
    let snaps = snaps();
    let r1 = snaps.iter().filter(|&&s| s < r).count();
    let g1 = snaps.iter().filter(|&&s| s < g).count();
    let b1 = snaps.iter().filter(|&&s| s < b).count();

    (r1 * 36) + (g1 * 6) + b1 + 16
}

pub fn calc_size(img: &image::DynamicImage, width: Option<u32>) -> Result<(u32, u32), &str> {
    let (original_height, original_width) = img.dimensions();
    let ratio: f64 = original_width as f64 / original_height as f64;

    match (width, terminal_size()) {
        (Some(w), _) => Ok(w),
        (None, Some((Width(w), _))) => Ok(w as u32),
        _ => Err("Unable to determine terminal size"),
    }
    .map(|w| (w as u32, (w as f64 * ratio) as u32))
}

pub fn true_color(img: &image::DynamicImage, x: u32, y: u32) -> String {
    let (r1, b1, g1, _a1) = img.get_pixel(x, y).channels4();
    let (r2, b2, g2, _a2) = img.get_pixel(x, y + 1).channels4();
    format!("\x1b[48;2;{};{};{};38;2;{};{};{}m▄", r1, b1, g1, r2, b2, g2)
}

pub fn indexed(img: &image::DynamicImage, x: u32, y: u32) -> String {
    let (r1, b1, g1, _a1) = img.get_pixel(x, y).channels4();
    let (r2, b2, g2, _a2) = img.get_pixel(x, y + 1).channels4();

    let bg = rgb2short(r1, b1, g1);
    let fg = rgb2short(r2, b2, g2);
    format!("\x1b[38;5;{};48;5;{}m▄", fg, bg)
}

pub fn from_str(input: &str) -> Result<FilterType, &str> {
    match input.to_lowercase().as_str() {
        "nearest" => Ok(FilterType::Nearest),
        "triangle" => Ok(FilterType::Triangle),
        "catmullrom" => Ok(FilterType::CatmullRom),
        "gaussian" => Ok(FilterType::Gaussian),
        "lanczos3" => Ok(FilterType::Lanczos3),
        _ => Err(input),
    }
}
