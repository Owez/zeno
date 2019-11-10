//! # About
//! 
//! Primary editor screen showing the TextArea/[Upgrade]
//! (https://gitlab.com/zeno-src/zeno/issues/3).

use crate::editor::save::save_as;
use crate::profile::{options::profile_options, Profile};
use crate::StartMeta;
use cursive::views::{BoxView, LinearLayout, OnEventView, ScrollView, TextArea, TextView};
use cursive::{event, traits::*, Cursive};
use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;

/// Shows the main editor screen.
pub fn editor_screen(s: &mut Cursive, p_name: &str, meta: &StartMeta) {
    s.pop_layer();

    let selected_profile = Rc::new(RefCell::new(Profile {
        name: String::from(p_name),
        theme: PathBuf::new(), // TODO load theme from db
    }));

    let text_enclosure = ScrollView::new(BoxView::with_full_screen(
        OnEventView::new(smart_text_area(meta).with_id("tb"))
            .on_pre_event(event::Event::CtrlChar('s'), save_as)
            .on_pre_event(event::Event::CtrlChar('l'), move |s| {
                profile_options(s, Rc::clone(&selected_profile));
            }),
    ));
    let save_info = TextView::new(
        "Save: ctrl+s, Exit: ctrl+c, HSplit: ctrl+[left/right], VSplit: ctrl+[up/down], Profile settings: ctrl+l",
    );

    s.add_fullscreen_layer(
        LinearLayout::vertical()
            .child(text_enclosure)
            .child(save_info),
    );
}

/// A "smart" text area that initializes depending on [StartMeta.open_path] (will
/// return a new, blank [TextArea] if no file to open or a pre-filled one if a
/// file was given).
fn smart_text_area(meta: &StartMeta) -> TextArea {
    /// Gets content from a specified file path ([PathBuf]) and returns a string
    /// or panics in the process.
    fn get_path_content(path: &PathBuf) -> String {
        let mut got_file = File::open(path).unwrap();
        let mut content = String::new();

        got_file.read_to_string(&mut content).unwrap();

        content
    }

    let text_area = TextArea::new();

    match &meta.open_path {
        None => text_area,
        Some(p) => text_area.content(get_path_content(p)),
    }
}
