use image::{ImageBuffer, Rgb, Rgba};

pub fn rgb_to_rgba(rgb_image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = rgb_image.dimensions();
    let mut rgba_image = ImageBuffer::new(width, height);
    for (x, y, pixel) in rgb_image.enumerate_pixels() {
        let Rgb([r, g, b]) = *pixel;
        let rgba = Rgba([r, g, b, 255]);
        rgba_image.put_pixel(x, y, rgba);
    }
    rgba_image
}
