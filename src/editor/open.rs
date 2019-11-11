//! # About
//!
//! Contains utility functions for reading contents of files and an interactive
//! cursive prompt to open a new file inside of Zeno, instead of using the usual
//! CLI.

use cursive::views::{Dialog, EditView, TextArea};
use cursive::{traits::*, Cursive};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

/// Displays a popup to open a file and adds it to the primary text area ID (`tb`).
pub fn open_path_str(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(EditView::new().with_id("open_file_name").fixed_width(32))
            .title("Open file")
            .button("Open", |s| {
                let got_path = s
                    .call_on_id("open_file_name", |view: &mut EditView| view.get_content())
                    .unwrap(); // Get path from EditView

                let contents = match get_content_str(&got_path) {
                    Ok(x) => x,
                    Err(e) => {
                        s.add_layer(Dialog::info(e));
                        return ();
                    },
                }; // Use [got_path] to get file contents or exit if fail

                s.call_on_id("tb", |view: &mut TextArea| view.set_content(contents))
                    .unwrap(); // Set textarea content
            })
            .button("Cancel", |s| {
                s.pop_layer();
            }),
    );
}

/// Gets content from a given &[str] and return file's contents or nicely error
/// when trying.
fn get_content_str(file_path: &str) -> Result<String, String> {
    let pb_p = PathBuf::from(file_path);

    if !pb_p.exists() {
        Err(format!("{:?} does not exist", file_path))
    } else if pb_p.is_dir() {
        Err(format!("{:?} is a directory", file_path))
    } else {
        Ok(get_path_content(&pb_p))
    }
}

/// Gets content from a specified file path ([PathBuf]) and returns a string
/// or panics in the process.
pub fn get_path_content(path: &PathBuf) -> String {
    let mut got_file = File::open(path).unwrap();
    let mut content = String::new();

    got_file.read_to_string(&mut content).unwrap();

    content
}
