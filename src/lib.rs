use cursive::views::{
    BoxView, Button, Dialog, EditView, LinearLayout, OnEventView, ScrollView, SelectView, TextArea,
};
use cursive::Cursive;
use cursive::{event, traits::*};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

/// Storage structure for holding metadata for a given profile in-memory.
pub struct Profile {
    /// Name of the profile
    pub name: String,
}

/// A structure for configuring the text editor before profile selecting.
///
/// This is useful for using in combination with a CLI to choose what to open
/// for example.
pub struct StartMeta {
    /// The path to automatically open after profile has been properly chosen.
    pub open_path: Option<PathBuf>,
}

/// Start of zeno's ui, enacting all basic functionality. You may pass in a file
/// to open automatically
pub fn zeno_launch(s: &mut Cursive, meta: StartMeta) {
    profile_select(s, meta);
}

/// Profile selector for multi-user/multi-purpose editing (allowing for more
/// flexible options).
fn profile_select(s: &mut Cursive, meta: StartMeta) {
    let profile_list = SelectView::<String>::new()
        .on_submit(move |s, selected_item| {
            editor_screen(s, selected_item, &meta);
        })
        .with_id("p_list")
        .fixed_size((32, 8));
    let admin_buttons = LinearLayout::vertical()
        .child(Button::new("Add new", add_profile))
        .child(Button::new("Remove", remove_conf));

    s.pop_layer();
    s.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(profile_list)
                .child(admin_buttons),
        )
        .title("Profile selector"),
    )
}

fn remove_conf(s: &mut Cursive) {
    s.add_layer(
        Dialog::text("Are you sure you want to delete the selected profile?")
            .button("Yes", remove_profile)
            .button("No", |s| {
                s.pop_layer();
            }),
    )
}

/// Allows a user to delete/remove a profile.
fn remove_profile(s: &mut Cursive) {
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
fn add_profile(s: &mut Cursive) {
    /// Adds a name to the profile list ([SelectView])
    fn add_to_list(s: &mut Cursive, p_name: &str) {
        if p_name == "" {
            s.add_layer(Dialog::info("Cannot add a new profile with no name!"));
        } else {
            s.call_on_id("p_list", |view: &mut SelectView<String>| {
                view.add_item_str(p_name);
            });
            s.pop_layer();
        }
    }

    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(add_to_list)
                .with_id("p_name")
                .fixed_width(32),
        )
        .title("Add new profile")
        .button("Ok", |s| {
            let p_name = s
                .call_on_id("p_name", |view: &mut EditView| view.get_content())
                .unwrap(); // Get content from EditView
            add_to_list(s, &p_name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }),
    )
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

/// Shows the main editor screen.
fn editor_screen(s: &mut Cursive, p_name: &str, meta: &StartMeta) {
    s.pop_layer();

    let _selected_profile = Profile {
        name: String::from(p_name),
    };

    let text_enclosure = ScrollView::new(BoxView::with_full_screen(
        OnEventView::new(smart_text_area(meta).with_id("tb"))
            .on_pre_event(event::Event::CtrlChar('s'), save_as),
    ));
    let save_info = TextArea::new()
        .content("Save: ctrl+s, Exit: ctrl+c, HSplit: ctrl+[left/right], VSplit: ctrl+[up/down]");

    s.add_fullscreen_layer(
        LinearLayout::vertical()
            .child(text_enclosure)
            .child(save_info),
    );
}

/// Dialog to find what user should save a given file as and then will attempt to save
fn save_as(s: &mut Cursive) {
    /// Dumps all inside editor to specified location
    fn save_file(s: &mut Cursive, file_name: &str, str_buf: &str) {
        let mut new_file = File::create(file_name).unwrap();
        new_file.write_all(str_buf.as_bytes()).unwrap();

        s.add_layer(Dialog::info(format!("Saved to '{}'!", file_name)));
    }

    s.add_layer(
        Dialog::around(EditView::new().with_id("file_name").fixed_width(32))
            .title("Save file as")
            .button("Save", |s| {
                let file_name = s
                    .call_on_id("file_name", |view: &mut EditView| view.get_content())
                    .unwrap(); // Get content from EditView
                let str_buf = s
                    .call_on_id("tb", |view: &mut TextArea| String::from(view.get_content()))
                    .unwrap(); // Get content from EditView
                save_file(s, &file_name, &str_buf);
            })
            .button("Cancel", |s| {
                s.pop_layer();
            }),
    );
}
