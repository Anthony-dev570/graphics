use std::collections::HashMap;
use std::sync::MutexGuard;
use mathematics::linear_algebra::matrix::types::Mat4F32;
use mathematics::linear_algebra::vector::types::{Vector2, Vector2F32};

use crate::vertex::pos2_uv::Position2Uv;
use crate::vertex::pos_uv_vertex::PositionUvVertex;
use mathematics::linear_algebra::vector::types::Vector3;
use crate::ui::style::{Style, StyleProperty, StyleValue};

pub mod input_field;
pub mod style;

pub(crate) static mut FOCUS: Option<usize> = None;
pub(crate) static mut UI_ELEMENTS: Vec<Box<dyn UI>> = vec![];

static mut INC_ID: usize = 0;

pub(crate) fn ui_id() -> usize {
    unsafe {
        let id = INC_ID;
        INC_ID += 1;
        id
    }
}

pub fn get_focused() -> Option<&'static Box<dyn UI>> {
    unsafe {
        for element in &UI_ELEMENTS {
            if element.has_focus() {
                return Some(element);
            }
        }
    }
    None
}

pub fn process_mouse_click(
    mouse_position: Vector2F32,
    mouse: u32,
) {
    unsafe {
        let elements = &mut UI_ELEMENTS;

        for i in 0..elements.len() {
            let i = elements.len() - 1 - i;

            if elements[i].contains_point(mouse_position) {
                elements[i].request_focus();
                elements[i].process_click(mouse_position, mouse);
                //FOCUS = Some(elements[i].id());
                return;
            }
        }
    }
}

pub fn process_key(
    key: u32,
    action: u32
) {
    unsafe {
        if let Some(s) = get_focused() {
            s.process_key(key, action);
        }
    }
}

pub fn send_lost_focus() {
    println!("Send it");
    if let Some(focused) = get_focused() {
        focused.on_lost_focus();
    }
}

pub trait UI {
    fn id(&self) -> usize;
    fn draw(&self, projection: &Mat4F32);
    fn size(&self) -> Vector2F32;
    fn position(&self) -> Vector2F32;

    fn process_click(&self, point: Vector2F32, mouse_button: u32) {

    }

    fn process_key(&self, key: u32, action: u32) {

    }

    fn contains_point(&self, v: Vector2F32) -> bool {
        let (x, y) = {
            let pos = self.position();
            (pos[0], pos[1])
        };

        let (w, h) = {
            let size = self.size();
            (size[0], size[1])
        };

        v[0] >= x && v[0] <= x + w && v[1] >= y && v[1] <= y + h
    }

    fn set_position(&self, position: Vector2F32);
    fn set_size(&self, size: Vector2F32);

    fn style(&self) -> Style;
    fn insert_style_property(&self, property: StyleProperty, value: StyleValue);

    fn request_focus(&self);
    fn on_lost_focus(&self) {}
    fn has_focus(&self) -> bool {
        unsafe {
            if let Some(focus) = &FOCUS {
                let id = self.id();
                if id == *focus {
                    return true;
                }
            }
        }
        false
    }

    fn build_rect3d(position: Vector2F32, size: Vector2F32) -> [PositionUvVertex; 4] where Self: Sized {
        let (x, y) = (position[0], position[1]);
        let (w, h) = (size[0], size[1]);
        let position = Vector3::new([x, y, 0_f32]);
        [
            PositionUvVertex {
                position,
                uv: Default::default(),
            },
            PositionUvVertex {
                position: Vector3::new([x + w, y, 0_f32]),
                uv: Vector2::new([1_f32, 0_f32]),
            },
            PositionUvVertex {
                position: Vector3::new([x + w, y + h, 0_f32]),
                uv: Vector2::new([1_f32, 1_f32]),
            },
            PositionUvVertex {
                position: Vector3::new([x, y + h, 0_f32]),
                uv: Vector2::new([0_f32, 1_f32]),
            },
        ]
    }

    fn build_rect(position: Vector2F32, size: Vector2F32) -> [Position2Uv; 4] where Self: Sized {
        let (x, y) = (position[0], position[1]);
        let (w, h) = (size[0], size[1]);
        [
            Position2Uv {
                position,
                uv: Default::default(),
            },
            Position2Uv {
                position: Vector2::new([x + w, y]),
                uv: Vector2::new([1_f32, 0_f32]),
            },
            Position2Uv {
                position: Vector2::new([x + w, y + h]),
                uv: Vector2::new([1_f32, 1_f32]),
            },
            Position2Uv {
                position: Vector2::new([x, y + h]),
                uv: Vector2::new([0_f32, 1_f32]),
            },
        ]
    }
}