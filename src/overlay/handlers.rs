use std::io::Cursor;
use std::sync::Arc;
use std::time::Instant;

use crate::overlay::helpers::load_font;
use crate::overlay::image::{Image, OverlayText, PositionType};
use crate::router::AppState;
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::error::AppError;

use super::image::BlendMode;

#[derive(Deserialize, Serialize)]
pub struct BookCoverParams {
    pub author_font: String,
    pub author: String,
    pub author_position: PositionType,
    pub title_font: String,
    pub title: String,
    pub title_position: PositionType,
    pub blend_mode: BlendMode,
    pub alfa: f32,
    pub image_url: String,
    pub line_length: u8,
}

#[axum_macros::debug_handler]
pub async fn book_cover(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BookCoverParams>,
) -> Result<Vec<u8>, AppError> {
    let url = payload.image_url;
    let mut image = Image::from_url(url.as_str(), state).await?;

    let title_splits = textwrap::wrap(payload.title.as_str(), payload.line_length as usize);

    let rev_title_splits = title_splits
        .into_iter()
        .map(|s| s.to_string())
        .rev()
        .collect::<Vec<String>>();

    let author_font = load_font(payload.author_font.as_str())?;
    let title_font = load_font(payload.title_font.as_str())?;

    let author = OverlayText {
        text_list: vec![payload.author],
        color: (255, 255, 255),
        offset: (0, 0),
        alpha: payload.alfa,
        font: author_font,
        position: payload.author_position,
        blend: payload.blend_mode,
    };

    let title = OverlayText {
        text_list: rev_title_splits.clone(),
        color: (255, 255, 255),
        offset: (0, 0),
        alpha: payload.alfa,
        font: title_font.clone(),
        position: payload.title_position,
        blend: payload.blend_mode,
    };

    image.put_text(author).put_text(title);

    let mut buf: Vec<u8> = Vec::new();
    image
        .dyn_img
        .write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png)?;
    Ok(buf)
}
