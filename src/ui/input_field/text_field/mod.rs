use mathematics::linear_algebra::vector::types::Vector2F32;
use crate::atomic::Atomic;
use crate::font::Font;
use crate::ui::style::Style;
use crate::ui::ui_id;

pub mod imp;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextFilter {
    NumbersOnly,
    Alphanumeric,
    AlphaOnly,
    Algebraic,
    Label
}

pub struct TextFieldInner {
    buffer: String,
    caret: usize,
    position: Vector2F32,
    size: Vector2F32,
    style: Style,
    id: usize,
    font: Option<Font>,
    text_filter: Option<TextFilter>,
    has_error: bool
}

impl Default for TextFieldInner {
    fn default() -> Self {
        Self {
            buffer: String::new(),
            caret: 0,
            position: Default::default(),
            size: Vector2F32::new([100.0, 20_f32]),
            style: Default::default(),
            id: ui_id(),
            font: None,
            text_filter: None,
            has_error: false
        }
    }
}

#[derive(Clone)]
pub struct TextField(Atomic<TextFieldInner>);