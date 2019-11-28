//! Profile options popup/tree to allow a given user to edit their profile on the
//! fly. See the main [profile_options] function for more information.

use crate::profile::Profile;
use crate::theme::get_themes;
use cursive::views::{Dialog, SelectView};
use cursive::Cursive;
use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;
use tinydb::Database;

/// Allows users to edit their profile options.
pub fn profile_options(
    s: &mut Cursive,
    profile: Rc<RefCell<Profile>>,
    p_db: Rc<RefCell<Database<Profile>>>,
) {
    let profile_theme_options = move |s: &mut Cursive| {
        let mut theme_select = SelectView::<PathBuf>::new().on_submit(move |s, theme| {
            load_theme(s, theme, Rc::clone(&profile), Rc::clone(&p_db));
        });

        for theme in get_themes().unwrap().iter() {
            theme_select.add_item(theme.nickname.clone(), theme.path.clone());
        }

        // s.add_layer(Dialog::info(format!("It works, {}", profile.borrow().name)));
        s.add_layer(Dialog::around(theme_select).title("Theme select"))
    };

    let options = SelectView::new()
        .item("Themes", profile_theme_options)
        .on_submit(|s, call| call(s));

    s.add_layer(
        Dialog::around(options)
            .button("Close", |s| {
                s.pop_layer();
            })
            .title("Profile settings"),
    );
}

fn load_theme(
    s: &mut Cursive,
    got_theme: &PathBuf,
    profile: Rc<RefCell<Profile>>,
    p_db: Rc<RefCell<Database<Profile>>>,
) {
    let file = match File::open(got_theme) {
        Ok(x) => Some(x),
        Err(_) => {
            s.add_layer(Dialog::info(format!(
                "Could not load theme {:?}!",
                got_theme
            )));
            None
        }
    };

    if file.is_some() {
        let mut content = String::new();
        file.unwrap().read_to_string(&mut content).unwrap();

        s.load_toml(&content).unwrap();

        let p_db_mut = p_db.borrow_mut();

        let base_profile = profile.clone().into_inner();
        let new_profile = profile.clone();

        new_profile.borrow_mut().theme = got_theme.to_path_buf();

        p_db_mut
            .update_item(&base_profile, new_profile.into_inner())
            .unwrap();
    }
}
