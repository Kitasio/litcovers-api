use std::io::Cursor;

use axum::Json;
use image::DynamicImage;
use image::{GenericImage, GenericImageView};
use rusttype::{point, Font, Scale};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

struct Shadow {
    colour: (u8, u8, u8),
    offset: (i32, i32),
}

#[derive(Deserialize, Serialize)]
pub struct OverlayParams {
    pub author_font: String,
    pub title_font: String,
    pub title: String,
    pub author: String,
    pub image_url: String,
    pub line_length: u8,
}

pub async fn overlay(Json(payload): Json<OverlayParams>) -> Result<Vec<u8>, AppError> {
    let splits = textwrap::wrap(payload.title.as_str(), payload.line_length as usize);

    let splits = splits
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let image = get_image(payload.image_url.as_str()).await?;

    let image = create_overlay_img(
        &payload.author_font,
        &payload.title_font,
        splits,
        &payload.author,
        &mut image.clone(),
    )?;

    let mut buf = Vec::new();
    image.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png)?;
    Ok(buf)
}

// calculates font size for a given width
fn calc_font_size(width: u32, text: &str, font: &Font) -> Scale {
    let mut scale = Scale::uniform(1.0);
    let mut glyph_width = 0.0;
    for c in text.chars() {
        glyph_width += font.glyph(c).scaled(scale).h_metrics().advance_width;
    }
    scale.x *= width as f32 / glyph_width;
    scale.y *= width as f32 / glyph_width;
    scale
}

fn calc_text_width(text: &str, font: &Font, scale: Scale) -> u32 {
    let mut glyph_width = 0.0;
    for c in text.chars() {
        glyph_width += font.glyph(c).scaled(scale).h_metrics().advance_width;
    }
    glyph_width as u32
}

// creates DynamicImage from image URL
pub async fn get_image(url: &str) -> Result<DynamicImage, AppError> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    Ok(image::load_from_memory(&bytes)?)
}

pub fn create_overlay_img(
    author_font: &String,
    title_font: &String,
    text_list: Vec<String>,
    author: &String,
    image: &mut DynamicImage,
) -> Result<DynamicImage, AppError> {
    let (img_width, img_height) = image.dimensions();

    let author_font_path = format!("./fonts/{}", author_font);
    let title_font_path = format!("./fonts/{}", title_font);

    let data = std::fs::read(&author_font_path)?;
    let author_font = if let Some(font) = Font::try_from_vec(data) {
        font
    } else {
        return Err(AppError::FontNotFound);
    };

    let data = std::fs::read(&title_font_path)?;
    let title_font = if let Some(font) = Font::try_from_vec(data) {
        font
    } else {
        return Err(AppError::FontNotFound);
    };

    let shadow = Shadow {
        colour: (0, 0, 0), // black shadow
        offset: (4, 4),    // offset the shadow by 10 pixel in x and y direction
    };

    let colour = (255, 255, 255);

    // reverses text list
    let text_list = text_list.into_iter().rev().collect::<Vec<String>>();

    let mut stacked_height: f32 = 0.0;
    let mut padding_y: u32 = 50;
    let padding_x: u32 = 50;

    // overlay author name at the top center of the image
    let author_scale = Scale::uniform(24.0);
    // offset to the center
    let offset = point(
        (img_width as f32 / 2.0) - calc_text_width(author, &author_font, author_scale) as f32 / 2.0,
        stacked_height + 40.0,
    );
    let author_glyphs: Vec<_> = author_font.layout(&author, author_scale, offset).collect();

    for g in author_glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|x, y, v| {
                println!("{}", v);
                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;
                if x >= 0 && x < img_width as i32 && y >= 0 && y < img_height as i32 {
                    let alpha = 0.5;
                    let c = image.get_pixel(x as u32, y as u32);
                    let c = image::Rgba([
                        (c[0] as f32 * (1.0 - alpha * v) + colour.0 as f32 * alpha * v) as u8,
                        (c[1] as f32 * (1.0 - alpha * v) + colour.1 as f32 * alpha * v) as u8,
                        (c[2] as f32 * (1.0 - alpha * v) + colour.2 as f32 * alpha * v) as u8,
                        255,
                    ]);
                    image.put_pixel(x as u32, y as u32, c);
                }
            });
        }
    }

    for text in text_list {
        let text = text.to_uppercase();

        let scale = calc_font_size(img_width - padding_x, &text, &title_font);
        let v_metrics = title_font.v_metrics(scale);

        let left = padding_x as f32 / 2.0;
        let top = img_height as f32 - stacked_height - padding_y as f32 / 2.0;

        let offset = point(left, top);

        // update stacked height
        stacked_height += v_metrics.ascent;
        // update padding y
        padding_y += 35;

        // layout the glyphs in a line
        let glyphs: Vec<_> = title_font.layout(&text, scale, offset).collect();

        // draw the shadow
        for g in glyphs.clone() {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|x, y, v| {
                    let x = x as i32 + bb.min.x + shadow.offset.0;
                    let y = y as i32 + bb.min.y + shadow.offset.1;
                    if x >= 0 && x < img_width as i32 && y >= 0 && y < img_height as i32 {
                        let c = image.get_pixel(x as u32, y as u32);
                        let c = image::Rgba([
                            (c[0] as f32 * (1.0 - v) + shadow.colour.0 as f32 * v) as u8,
                            (c[1] as f32 * (1.0 - v) + shadow.colour.1 as f32 * v) as u8,
                            (c[2] as f32 * (1.0 - v) + shadow.colour.2 as f32 * v) as u8,
                            255,
                        ]);
                        image.put_pixel(x as u32, y as u32, c);
                    }
                });
            }
        }

        // put text on test image
        for g in glyphs {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|x, y, v| {
                    let x = x as i32 + bb.min.x;
                    let y = y as i32 + bb.min.y;
                    if x >= 0 && x < img_width as i32 && y >= 0 && y < img_height as i32 {
                        let c = image.get_pixel(x as u32, y as u32);
                        let c = image::Rgba([
                            (c[0] as f32 * (1.0 - v) + colour.0 as f32 * v) as u8,
                            (c[1] as f32 * (1.0 - v) + colour.1 as f32 * v) as u8,
                            (c[2] as f32 * (1.0 - v) + colour.2 as f32 * v) as u8,
                            255,
                        ]);
                        image.put_pixel(x as u32, y as u32, c);
                    }
                });
            }
        }
    }

    let mut img_clone = DynamicImage::new_rgb16(img_width, img_height);
    image.clone_into(&mut img_clone);
    Ok(img_clone)
}
