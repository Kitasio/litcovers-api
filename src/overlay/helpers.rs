use crate::error::AppError;
use image::DynamicImage;
use image::{GenericImage, GenericImageView};
use rusttype::{Font, PositionedGlyph, Scale};

const FONTS_DIR: &'static str = "fonts";

// calculates font size for a given width
pub fn calc_font_size(width: u32, text: &str, font: &Font) -> Scale {
    let mut scale = Scale::uniform(1.0);
    let mut glyph_width = 0.0;
    for c in text.chars() {
        glyph_width += font.glyph(c).scaled(scale).h_metrics().advance_width;
    }
    scale.x *= width as f32 / glyph_width;
    scale.y *= width as f32 / glyph_width;
    scale
}

pub fn calc_text_width(text: &str, font: &Font, scale: Scale) -> u32 {
    let mut glyph_width = 0.0;
    for c in text.chars() {
        glyph_width += font.glyph(c).scaled(scale).h_metrics().advance_width;
    }
    glyph_width as u32
}

pub fn load_font(font_file_name: &str) -> Result<Font<'static>, AppError> {
    let font_path = format!("{}/{}", FONTS_DIR, font_file_name);
    let font_file_data = std::fs::read(&font_path)?;
    match Font::try_from_vec(font_file_data) {
        Some(font) => Ok(font),
        None => Err(AppError::FontNotFound),
    }
}

pub fn draw_glyphs(
    glyphs: Vec<PositionedGlyph>,
    alpha: f32,
    color: (u8, u8, u8),
    offset: (i32, i32),
    mut image: DynamicImage,
) -> DynamicImage {
    let (img_width, img_height) = image.dimensions();
    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|x, y, v| {
                let x = x as i32 + bb.min.x + offset.0;
                let y = y as i32 + bb.min.y + offset.1;
                if x >= 0 && x < img_width as i32 && y >= 0 && y < img_height as i32 {
                    let c = image.get_pixel(x as u32, y as u32);
                    let c = image::Rgba([
                        (c[0] as f32 * (1.0 - alpha * v) + color.0 as f32 * alpha * v) as u8,
                        (c[1] as f32 * (1.0 - alpha * v) + color.1 as f32 * alpha * v) as u8,
                        (c[2] as f32 * (1.0 - alpha * v) + color.2 as f32 * alpha * v) as u8,
                        255,
                    ]);
                    image.put_pixel(x as u32, y as u32, c);
                }
            });
        }
    }
    image
}
