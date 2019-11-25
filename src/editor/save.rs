//! # About
//!
//! Save/dump utility for editor. Gives a prompt to save to a new file name
//! (hence [save_as] being called this way) and proceeds to dump the file.

use cursive::views::{Dialog, EditView, TextArea};
use cursive::{traits::*, Cursive};
use std::fs::File;
use std::io::prelude::*;

/// Dialog to find what user should save a given file as and then will attempt to save
pub fn save_as(s: &mut Cursive) {
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
                dump_file(s, &file_name, &str_buf);
            })
            .button("Cancel", |s| {
                s.pop_layer();
            }),
    );
}

/// Dumps all inside editor to specified location
fn dump_file(s: &mut Cursive, file_name: &str, str_buf: &str) {
    let mut new_file = File::create(file_name).unwrap();
    new_file.write_all(str_buf.as_bytes()).unwrap();

    s.add_layer(Dialog::info(format!("Saved to '{}'!", file_name)));
}
