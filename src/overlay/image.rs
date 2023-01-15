use std::sync::Arc;
use std::time::Duration;

use crate::error::AppError;
use crate::overlay::helpers::{calc_font_size, calc_text_width, kill_after};
use crate::router::AppState;
use image::DynamicImage;
use image::{GenericImage, GenericImageView};
use rusttype::{point, Font, PositionedGlyph, Scale};
use serde::{Deserialize, Serialize};

use super::helpers::longest_str;

pub struct OverlayText {
    pub text_list: Vec<String>,
    pub color: (u8, u8, u8),
    pub offset: (i32, i32),
    pub alpha: f32,
    pub font: Font<'static>,
    pub position: PositionType,
    pub blend: BlendMode,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum PositionType {
    TopCenter,
    BottomStretch,
    BottomSides,
    BottomLeft,
    BottomCenter,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum BlendMode {
    None,
    Overlay,
}

#[derive(Debug)]
pub struct Image {
    pub dyn_img: DynamicImage,
    pub url: String,
}

impl Image {
    pub fn put_text(&mut self, overlay: OverlayText) -> &mut Image {
        let (img_width, img_height) = self.dyn_img.dimensions();
        let mut stacked_height: f32 = 0.0;
        let mut padding_t: u32 = 50;
        let padding_l: u32 = 50;

        match overlay.position {
            PositionType::TopCenter => {
                stacked_height += 20.0;
                for text in overlay.text_list {
                    let scale = Scale::uniform(24.0);
                    let v_metrics = overlay.font.v_metrics(scale);

                    let left = (img_width as f32 / 2.0)
                        - calc_text_width(text.as_str(), &overlay.font, scale) as f32 / 2.0;
                    let top = stacked_height + padding_t as f32 / 2.0;

                    let offset = point(left, top);

                    let glyphs: Vec<PositionedGlyph> =
                        overlay.font.layout(&text, scale, offset).collect();

                    self.dyn_img = draw_glyphs(
                        glyphs,
                        overlay.alpha,
                        overlay.color,
                        overlay.offset,
                        overlay.blend,
                        self.dyn_img.clone(),
                    );

                    // update stacked height
                    stacked_height += v_metrics.ascent;
                    // update padding y
                    padding_t += 35;
                }
                self
            }
            PositionType::BottomStretch => {
                for text in overlay.text_list {
                    let text = text.to_uppercase();
                    let scale = calc_font_size(img_width - padding_l, &text, &overlay.font);
                    let v_metrics = overlay.font.v_metrics(scale);

                    let left = padding_l as f32 / 2.0;
                    let top = img_height as f32 - stacked_height - padding_t as f32 / 2.0;

                    let offset = point(left, top);

                    let glyphs: Vec<PositionedGlyph> =
                        overlay.font.layout(&text, scale, offset).collect();

                    self.dyn_img = draw_glyphs(
                        glyphs,
                        overlay.alpha,
                        overlay.color,
                        overlay.offset,
                        overlay.blend,
                        self.dyn_img.clone(),
                    );

                    // update stacked height
                    stacked_height += v_metrics.ascent;
                    // update padding y
                    padding_t += 35;
                }
                self
            }
            PositionType::BottomSides => {
                let mut left_side = true;
                for text in overlay.text_list {
                    let text = text.to_uppercase();
                    let scale = Scale::uniform(56.0);
                    let v_metrics = overlay.font.v_metrics(scale);

                    let offset = if left_side {
                        let left = padding_l as f32 / 2.0;
                        let top = img_height as f32 - stacked_height - padding_t as f32 / 2.0;
                        point(left, top)
                    } else {
                        let left = img_width as f32
                            - padding_l as f32 / 2.0
                            - calc_text_width(text.as_str(), &overlay.font, scale) as f32;
                        let top = img_height as f32 - stacked_height - padding_t as f32 / 2.0;
                        point(left, top)
                    };

                    let glyphs: Vec<PositionedGlyph> =
                        overlay.font.layout(&text, scale, offset).collect();

                    self.dyn_img = draw_glyphs(
                        glyphs,
                        overlay.alpha,
                        overlay.color,
                        overlay.offset,
                        overlay.blend,
                        self.dyn_img.clone(),
                    );

                    // update stacked height
                    stacked_height += v_metrics.ascent;
                    // update padding y
                    padding_t += 35;
                    // update left side
                    left_side = !left_side;
                }
                self
            }
            PositionType::BottomLeft => {
                let longest_line = longest_str(&overlay.text_list);
                let scale = calc_font_size(img_width - padding_l, &longest_line, &overlay.font);
                for text in overlay.text_list {
                    let text = text.to_uppercase();
                    let v_metrics = overlay.font.v_metrics(scale);

                    let offset = {
                        let left = padding_l as f32 / 2.0;
                        let top = img_height as f32 - stacked_height - padding_t as f32 / 2.0;
                        point(left, top)
                    };

                    let glyphs: Vec<PositionedGlyph> =
                        overlay.font.layout(&text, scale, offset).collect();

                    self.dyn_img = draw_glyphs(
                        glyphs,
                        overlay.alpha,
                        overlay.color,
                        overlay.offset,
                        overlay.blend,
                        self.dyn_img.clone(),
                    );

                    // update stacked height
                    stacked_height += v_metrics.ascent;
                    // update padding y
                    padding_t += 35;
                }
                self
            }
            PositionType::BottomCenter => {
                let longest_line = longest_str(&overlay.text_list);
                let scale = calc_font_size(img_width - padding_l, &longest_line, &overlay.font);
                for text in overlay.text_list {
                    let text = text.to_uppercase();
                    let v_metrics = overlay.font.v_metrics(scale);

                    let offset = {
                        let left = (img_width as f32 / 2.0)
                            - calc_text_width(text.as_str(), &overlay.font, scale) as f32 / 2.0;
                        let top = img_height as f32 - stacked_height - padding_t as f32 / 2.0;
                        point(left, top)
                    };

                    let glyphs: Vec<PositionedGlyph> =
                        overlay.font.layout(&text, scale, offset).collect();

                    self.dyn_img = draw_glyphs(
                        glyphs,
                        overlay.alpha,
                        overlay.color,
                        overlay.offset,
                        overlay.blend,
                        self.dyn_img.clone(),
                    );

                    // update stacked height
                    stacked_height += v_metrics.ascent;
                    // update padding y
                    padding_t += 35;
                }
                self
            }
        }
    }

    // creates Image from image URL
    pub async fn from_url(url: &str, state: Arc<AppState>) -> Result<Image, AppError> {
        let mut img_bytes: Vec<u8> = Vec::new();
        {
            let img_map = state.images.lock().unwrap();
            img_bytes = match img_map.get(&url.to_string()) {
                Some(bytes) => bytes.clone(),
                None => img_bytes,
            }
        }
        if img_bytes.len() > 0 {
            let image = image::load_from_memory(&img_bytes)?;
            Ok(Image {
                dyn_img: image,
                url: url.to_string(),
            })
        } else {
            let response = reqwest::get(url).await?;
            let bytes = response.bytes().await?;
            {
                let mut img_map = state.images.lock().unwrap();
                img_map
                    .entry(url.to_string())
                    .or_insert(Vec::from(bytes.clone()));
            }
            kill_after(Duration::from_secs(120), state, url.to_string()).await;
            let image = image::load_from_memory(&bytes)?;
            Ok(Image {
                dyn_img: image,
                url: url.to_string(),
            })
        }
    }

    pub fn blend_mode(
        mode: BlendMode,
        pixel_rgb: (u8, u8, u8),
        color_rgb: (u8, u8, u8),
        alpha: f32,
        v: f32,
    ) -> [u8; 4] {
        match mode {
            BlendMode::None => [
                (pixel_rgb.0 as f32 * (1.0 - alpha * v) + color_rgb.0 as f32 * alpha * v) as u8,
                (pixel_rgb.1 as f32 * (1.0 - alpha * v) + color_rgb.1 as f32 * alpha * v) as u8,
                (pixel_rgb.2 as f32 * (1.0 - alpha * v) + color_rgb.2 as f32 * alpha * v) as u8,
                255,
            ],
            BlendMode::Overlay => {
                let r = if (pixel_rgb.0 as f32) < 128.0 {
                    2.0 * pixel_rgb.0 as f32 * color_rgb.0 as f32 / 255.0
                } else {
                    255.0
                        - 2.0 * (255.0 - pixel_rgb.0 as f32) * (255.0 - color_rgb.0 as f32) / 255.0
                };
                let g = if (pixel_rgb.1 as f32) < 128.0 {
                    2.0 * pixel_rgb.1 as f32 * color_rgb.1 as f32 / 255.0
                } else {
                    255.0
                        - 2.0 * (255.0 - pixel_rgb.1 as f32) * (255.0 - color_rgb.1 as f32) / 255.0
                };
                let b = if (pixel_rgb.2 as f32) < 128.0 {
                    2.0 * pixel_rgb.2 as f32 * color_rgb.2 as f32 / 255.0
                } else {
                    255.0
                        - 2.0 * (255.0 - pixel_rgb.2 as f32) * (255.0 - color_rgb.2 as f32) / 255.0
                };
                [
                    (pixel_rgb.0 as f32 * (1.0 - alpha * v) + r as f32 * alpha * v) as u8,
                    (pixel_rgb.1 as f32 * (1.0 - alpha * v) + g as f32 * alpha * v) as u8,
                    (pixel_rgb.2 as f32 * (1.0 - alpha * v) + b as f32 * alpha * v) as u8,
                    255,
                ]
            }
        }
    }
}

pub fn draw_glyphs(
    glyphs: Vec<PositionedGlyph>,
    alpha: f32,
    color: (u8, u8, u8),
    offset: (i32, i32),
    mode: BlendMode,
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
                    let rgba = Image::blend_mode(mode, (c[0], c[1], c[2]), color, alpha, v);
                    let c = image::Rgba(rgba);
                    image.put_pixel(x as u32, y as u32, c);
                }
            });
        }
    }
    image
}
