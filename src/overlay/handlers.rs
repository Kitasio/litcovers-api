use std::io::Cursor;

use crate::overlay::helpers::load_font;
use crate::overlay::image::{Image, OverlayText, PositionType};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::error::AppError;

use super::helpers::less_than_five_chars;
use super::image::BlendMode;

#[derive(Deserialize, Serialize)]
pub struct BookCoverParams {
    pub author_font: String,
    pub title_font: String,
    pub title: String,
    pub author: String,
    pub image_url: String,
    pub line_length: u8,
}

pub async fn book_cover(Json(payload): Json<BookCoverParams>) -> Result<Vec<u8>, AppError> {
    let title_splits = textwrap::wrap(payload.title.as_str(), payload.line_length as usize);

    let rev_title_splits = title_splits
        .into_iter()
        .map(|s| s.to_string())
        .rev()
        .collect::<Vec<String>>();

    let mut image = Image::from_url(payload.image_url.as_str()).await?;
    let author_font = load_font(payload.author_font.as_str())?;
    let title_font = load_font(payload.title_font.as_str())?;

    let title_position = if less_than_five_chars(rev_title_splits.clone()) {
        PositionType::BottomSides
    } else {
        PositionType::BottomStretch
    };

    let author = OverlayText {
        text_list: vec![payload.author],
        color: (255, 255, 255),
        offset: (0, 0),
        alpha: 1.0,
        font: author_font,
        position: PositionType::TopCenter,
        blend: BlendMode::None,
    };

    let title = OverlayText {
        text_list: rev_title_splits.clone(),
        color: (255, 255, 255),
        offset: (0, 0),
        alpha: 2.0,
        font: title_font.clone(),
        position: title_position.clone(),
        blend: BlendMode::Overlay,
    };

    image.put_text(author).put_text(title);

    let mut buf = Vec::new();
    image
        .0
        .write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png)?;
    Ok(buf)
}
