//! Primary editor screen showing the TextArea/[Upgrade](https://gitlab.com/zeno-src/zeno/issues/3).

use crate::editor::open::{get_path_content, open_path_str};
use crate::editor::save::save_as;
use crate::profile::{options::profile_options, Profile};
use crate::theme::push_toml_theme;
use crate::{utils, StartMeta};
use cursive::views::{BoxView, LinearLayout, OnEventView, ScrollView, TextArea, TextView};
use cursive::{event, traits::*, Cursive};
use std::cell::RefCell;
use std::rc::Rc;
use tinydb::Database;

/// Shows the main editor screen.
pub fn editor_screen(
    s: &mut Cursive,
    p_db: Rc<RefCell<Database<Profile>>>,
    p_name: &str,
    meta: &StartMeta,
) {
    s.pop_layer();

    let p_db_closure = Rc::clone(&p_db);

    let selected_profile = utils::find_profile(p_db, p_name);
    let selected_profile_ref = Rc::new(RefCell::new(selected_profile.clone()));

    let text_enclosure = ScrollView::new(BoxView::with_full_screen(
        OnEventView::new(smart_text_area(meta).with_id("tb"))
            .on_pre_event(event::Event::CtrlChar('s'), save_as)
            .on_pre_event(event::Event::CtrlChar('o'), open_path_str)
            .on_pre_event(event::Event::CtrlChar('l'), move |s| {
                profile_options(
                    s,
                    Rc::clone(&selected_profile_ref),
                    Rc::clone(&p_db_closure),
                );
            }),
    ));
    let save_info =
        TextView::new("Save: ctrl+s, Open: ctrl+o, Exit: ctrl+c, Profile settings: ctrl+l");

    s.add_fullscreen_layer(
        LinearLayout::vertical()
            .child(text_enclosure)
            .child(save_info),
    );

    push_toml_theme(s, selected_profile.clone().theme, false); // Load theme (don't message on success)
}

/// A "smart" text area that initializes depending on [StartMeta.open_path] (will
/// return a new, blank [TextArea] if no file to open or a pre-filled one if a
/// file was given).
fn smart_text_area(meta: &StartMeta) -> TextArea {
    let text_area = TextArea::new();

    match &meta.open_path {
        None => text_area,
        Some(p) => text_area.content(get_path_content(p)),
    }
}
