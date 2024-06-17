use std::collections::HashMap;
use std::path::Path;
use freetype::Library;

use crate::atomic::Atomic;
use crate::font::{Font, FontHandler};
use crate::font::font_type::FontType;

static mut LIBRARY: Option<Library> = None;

impl Font {
    pub fn new<N: ToString, P: AsRef<Path> + ToString>(
        name: N,
        font_types: HashMap<FontType, P>,
        render_font_size: i32,
    ) -> Self {
        Self(Atomic::new(FontHandler::Uninitialized {
            name: name.to_string(),
            font_types: font_types.into_iter().map(|(k, v)| (k, v.to_string())).collect::<HashMap<FontType, String>>(),
            render_font_size,
        }))
    }

    pub fn is_initialized(&self) -> bool {
        match &*self.0.0.lock().unwrap() {
            FontHandler::Uninitialized { .. } => false,
            FontHandler::Initialized(_) => true
        }
    }

    pub fn initialize(&self) {
        let mut ptr = self.0.0.lock().unwrap();
        if let FontHandler::Uninitialized { name, font_types, render_font_size } = &*ptr {
            unsafe {
                let library = Self::load_library();
            }
        }
    }

    fn load_library() -> &'static Library {
        unsafe {
            match &LIBRARY {
                None => {
                    LIBRARY = Some(Library::init().unwrap());
                    Self::load_library()
                }
                Some(library) => library
            }
        }
    }
}