use std::collections::HashMap;
use mathematics::color::Color;

#[derive(Debug, Clone, Copy)]
#[derive(Eq, Hash, PartialEq)]
pub enum StyleProperty {
    BackgroundColor,
    ForegroundColor
}

#[derive(Debug, Clone, Copy)]
pub enum StyleValue {
    Color(Color),
    Float(f32)
}

#[derive(Clone)]
pub struct Style(HashMap<StyleProperty, StyleValue>);

impl Style {
    pub fn put_property(&mut self, property: StyleProperty, value: StyleValue) {
        self.0.insert(property, value);
    }
}

impl Default for Style {
    fn default() -> Self {
        Self(HashMap::default())
    }
}