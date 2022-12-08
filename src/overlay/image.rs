use image::DynamicImage;

use crate::error::AppError;
use crate::overlay::helpers::{calc_font_size, calc_text_width, draw_glyphs};
use crate::overlay::text::{OverlayText, PositionType};
use image::GenericImageView;
use rusttype::{point, PositionedGlyph, Scale};

pub struct Image(pub DynamicImage);

impl Image {
    pub fn put_text(&mut self, overlay: OverlayText) -> &mut Image {
        let (img_width, img_height) = self.0.dimensions();
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

                    self.0 = draw_glyphs(
                        glyphs,
                        overlay.alpha,
                        overlay.color,
                        overlay.offset,
                        self.0.clone(),
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

                    self.0 = draw_glyphs(
                        glyphs,
                        overlay.alpha,
                        overlay.color,
                        overlay.offset,
                        self.0.clone(),
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

    // creates DynamicImage from image URL
    pub async fn from_url(url: &str) -> Result<Image, AppError> {
        let response = reqwest::get(url).await?;
        let bytes = response.bytes().await?;
        let image = image::load_from_memory(&bytes)?;
        Ok(Image(image))
    }
}
