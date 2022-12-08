use rusttype::Font;

pub struct OverlayText {
    pub text_list: Vec<String>,
    pub color: (u8, u8, u8),
    pub offset: (i32, i32),
    pub alpha: f32,
    pub font: Font<'static>,
    pub position: PositionType,
}

pub enum PositionType {
    TopCenter,
    BottomStretch,
}
