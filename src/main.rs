use cursive::traits::*;
use cursive::views::{
    BoxView, Button, Dialog, EditView, LinearLayout, SelectView, TextArea, TextView,
};
use cursive::Cursive;

/// Storage structure for holding metadata for a given profile in-memory.
struct Profile {
    /// Name of the profile
    name: String,
}

/// Start of zeno's ui, enacting all basic functionality.
fn zeno_launch(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(TextView::new(
            "This is a work-in-progress program and is not guaranteed to work.",
        ))
        .title("Welcome to Zeno")
        .button("Continue", profile_select)
        .button("Quit", |s| s.quit()),
    );
}

/// Profile selector for multi-user/multi-purpose editing (allowing for more
/// flexible options).
fn profile_select(s: &mut Cursive) {
    let profile_list = SelectView::<String>::new()
        .on_submit(editor_screen)
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
        s.call_on_id("p_list", |view: &mut SelectView<String>| {
            view.add_item_str(p_name);
        });
        s.pop_layer();
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

/// Shows the main editor screen.
fn editor_screen(s: &mut Cursive, p_name: &str) {
    s.pop_layer();

    // let selected_profile = Profile {
    //     name: String::from(p_name),
    // };

    s.add_layer(BoxView::with_full_screen(TextArea::new()));
}

fn main() {
    let mut siv = Cursive::default();

    siv.add_global_callback('e', |s| s.quit());
    zeno_launch(&mut siv);
    siv.run();
}
