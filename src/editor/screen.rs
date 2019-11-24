//! # About
//!
//! Primary editor screen showing the TextArea/[Upgrade]
//! (https://gitlab.com/zeno-src/zeno/issues/3).

use crate::editor::open::{get_path_content, open_path_str};
use crate::editor::save::save_as;
use crate::profile::{options::profile_options, Profile};
use crate::StartMeta;
use cursive::views::{BoxView, LinearLayout, OnEventView, ScrollView, TextArea, TextView};
use cursive::{event, traits::*, Cursive};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use tinydb::{error, Database};

/// Shows the main editor screen.
pub fn editor_screen(s: &mut Cursive, p_name: &str, meta: &StartMeta) {
    s.pop_layer();

    let db_path = PathBuf::from("data/db/profile.db");

    let mut p_db: Database<Profile> = match db_path.exists() {
        true => Database::from(db_path).unwrap(),
        false => Database::new(String::from("profile"), Some(db_path), true),
    };

    let selected_profile = find_or_make_profile(&mut p_db, p_name).unwrap();
    let selected_profile_ref = Rc::new(RefCell::new(selected_profile));

    let text_enclosure = ScrollView::new(BoxView::with_full_screen(
        OnEventView::new(smart_text_area(meta).with_id("tb"))
            .on_pre_event(event::Event::CtrlChar('s'), save_as)
            .on_pre_event(event::Event::CtrlChar('o'), open_path_str)
            .on_pre_event(event::Event::CtrlChar('l'), move |s| {
                profile_options(s, Rc::clone(&selected_profile_ref));
            }),
    ));
    let save_info =
        TextView::new("Save: ctrl+s, Open: ctrl+o, Exit: ctrl+c, Profile settings: ctrl+l");

    s.add_fullscreen_layer(
        LinearLayout::vertical()
            .child(text_enclosure)
            .child(save_info),
    );
}

/// Searches a database for a profile given it's name. If it exists, the function
/// will return it's details. If not, it will create one inside of the database
/// and return a new [Profile] from this function.
fn find_or_make_profile(
    p_db: &mut Database<Profile>,
    search_name: &str,
) -> Result<Profile, error::QueryError> {
    match p_db.query_item(|q: &Profile| &q.name, String::from(search_name)) {
        Ok(profile) => Ok(profile.clone()),
        Err(error::QueryError::ItemNotFound) => {
            let new_profile = Profile {
                name: String::from(search_name),
                theme: PathBuf::from("data/themes/dark-mode.toml"),
            };

            p_db.add_item(new_profile.clone()).unwrap();
            p_db.dump_db().unwrap();

            Ok(new_profile)
        }
        Err(e) => Err(e),
    }
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
