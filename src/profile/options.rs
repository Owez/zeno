//! Profile options popup/tree to allow a given user to edit their profile on the
//! fly. See the main [profile_options] function for more information.

use crate::profile::Profile;
use crate::theme::get_themes;
use cursive::views::{Dialog, SelectView};
use cursive::Cursive;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

/// Allows users to edit their profile options.
pub fn profile_options(s: &mut Cursive, _profile: Rc<RefCell<Profile>>) {
    let profile_theme_options = move |s: &mut Cursive| {
        let mut theme_select = SelectView::<PathBuf>::new();

        for theme in get_themes().unwrap().iter() {
            theme_select.add_item(theme.nickname.clone(), theme.path.clone());
        }

        // s.add_layer(Dialog::info(format!("It works, {}", profile.borrow().name)));
        s.add_layer(Dialog::around(theme_select))
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
