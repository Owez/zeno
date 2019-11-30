//! Profile options popup/tree to allow a given user to edit their profile on the
//! fly. See the main [profile_options] function for more information.

use crate::profile::Profile;
use crate::theme::{get_themes, load_theme, Theme};
use cursive::views::{BoxView, Dialog, ScrollView, SelectView};
use cursive::Cursive;
use std::cell::RefCell;
use std::rc::Rc;
use tinydb::Database;

/// Allows users to edit their profile options.
pub fn profile_options(
    s: &mut Cursive,
    profile: Rc<RefCell<Profile>>,
    p_db: Rc<RefCell<Database<Profile>>>,
) {
    let profile_theme_options = move |s: &mut Cursive| {
        let found_themes = match get_themes() {
            Ok(x) => Some(x),
            Err(_) => {
                s.add_layer(Dialog::info("Could not retrive any themes!"));
                None
            }
        };

        if found_themes.is_some() {
            let mut theme_select = SelectView::<Theme>::new().on_submit(move |s, theme| {
                load_theme(s, theme, Rc::clone(&profile), Rc::clone(&p_db))
            });

            for theme in found_themes.unwrap().iter() {
                let cloned_theme = theme.clone();
                theme_select.add_item(theme.nickname.clone(), cloned_theme);
            }

            s.add_layer(
                Dialog::around(BoxView::with_fixed_width(32, ScrollView::new(theme_select)))
                    .title("Theme select")
                    .button("Cancel", |s| {
                        s.pop_layer();
                    }),
            );
        }
    };

    let options = SelectView::new()
        .item("Themes", profile_theme_options)
        .on_submit(move |s, call| {
            let call_cloned = call.clone();
            call_cloned(s)
        });

    s.add_layer(
        Dialog::around(options)
            .button("Close", |s| {
                s.pop_layer();
            })
            .title("Profile settings"),
    );
}
