use crate::error::AppError;
use rusttype::{Font, Scale};
use unicode_segmentation::UnicodeSegmentation;

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

pub fn less_than(num: usize, text_list: Vec<String>) -> bool {
    for text in text_list {
        if text.graphemes(true).count() < num {
            return true;
        }
    }
    false
}
