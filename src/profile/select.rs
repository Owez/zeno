//! # About
//!
//! Basic profile management for the user, gives a popup for frontend CRD (create,
//! read, delete) operations on a profile.
//!
//! If you'd like profile *editing* options, please see [crate::profile::options].

use crate::editor::screen::editor_screen;
use crate::profile::Profile;
use crate::StartMeta;
use cursive::views::{Button, Dialog, EditView, LinearLayout, SelectView};
use cursive::{traits::*, Cursive};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use tinydb::Database;

/// Profile selector for multi-user/multi-purpose editing (allowing for more
/// flexible options).
pub fn profile_select(s: &mut Cursive, meta: StartMeta) {
    let db_path = PathBuf::from("data/profile.tinydb"); // path to open database

    let p_db = Rc::new(RefCell::new(match db_path.exists() {
        true => Database::from(db_path).unwrap(),
        false => Database::new(String::from("profile"), Some(db_path), true),
    })); // profile database

    let p_db_closure = Rc::clone(&p_db); // Scoping issues with p_db and moving closures.
    let p_db_closure_2 = Rc::clone(&p_db); // Scoping issues with p_db and moving closures again.
    let p_db_closure_3 = Rc::clone(&p_db); // Scoping issues with p_db and moving closures again.

    let mut profile_list = SelectView::<String>::new().on_submit(move |s, selected_item| {
        editor_screen(s, Rc::clone(&p_db_closure), selected_item, &meta);
    });
    let admin_buttons = LinearLayout::vertical()
        .child(Button::new("Add new", move |s| {
            add_profile(s, Rc::clone(&p_db_closure_2));
        }))
        .child(Button::new("Remove", move |s| { remove_conf(s, Rc::clone(&p_db_closure_3)) }));

    for profile in p_db.borrow().read_db().iter() {
        profile_list.add_item_str(profile.name.clone());
    } // add profiles to list

    s.pop_layer();
    s.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(profile_list.with_id("p_list").fixed_size((32, 8)))
                .child(admin_buttons),
        )
        .title("Profile selector"),
    )
}

/// Confirmation popup to make sure user really wants to delete their profile.
///
/// This should ideally be embedded *inside* of [remove_profile] in the future.
/// See <https://gitlab.com/zeno-src/zeno/issues/9> for more infomation.
fn remove_conf(s: &mut Cursive, p_db: Rc<RefCell<Database<Profile>>>) {
    s.add_layer(
        Dialog::text("Are you sure you want to delete the selected profile?")
            .button("Yes", move |s| {remove_profile(s, Rc::clone(&p_db));})
            .button("No", |s| {
                s.pop_layer();
            }),
    )
}

/// Allows a user to delete/remove a profile.
fn remove_profile(s: &mut Cursive, _p_db: Rc<RefCell<Database<Profile>>>) {
    s.pop_layer();

    let mut got_select = s.find_id::<SelectView<String>>("p_list").unwrap();

    match got_select.selected_id() {
        None => s.add_layer(Dialog::info("No profiles to remove!")),
        Some(profile) => {
            got_select.remove_item(profile);
        }
    }
}

/// Allows a user to create a new profile.
fn add_profile(s: &mut Cursive, p_db: Rc<RefCell<Database<Profile>>>) {
    /// Adds a name to the profile list ([SelectView])
    fn add_to_list(s: &mut Cursive, p_name: &str, p_db: Rc<RefCell<Database<Profile>>>) {
        if p_name == "" {
            s.add_layer(Dialog::info("Cannot add a new profile with no name!"));
        } else {
            s.call_on_id("p_list", |view: &mut SelectView<String>| {
                view.add_item_str(p_name);

                let mut p_db_mut = p_db.borrow_mut();

                p_db_mut
                    .add_item(Profile {
                        name: p_name.to_string(),
                        theme: PathBuf::from("data/themes/dark-theme.toml"),
                    })
                    .unwrap();
                p_db_mut.dump_db().unwrap();
            });
            s.pop_layer();
        }
    }

    let p_db_closure = Rc::clone(&p_db); // Scoping issues with p_db and moving closures

    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(move |s, selected_item| {
                    add_to_list(s, selected_item, Rc::clone(&p_db_closure))
                })
                .with_id("p_name")
                .fixed_width(32),
        )
        .title("Add new profile")
        .button("Ok", move |s| {
            let p_name = s
                .call_on_id("p_name", |view: &mut EditView| view.get_content())
                .unwrap(); // Get content from EditView
            add_to_list(s, &p_name, Rc::clone(&p_db));
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }),
    )
}
