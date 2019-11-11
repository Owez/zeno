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
use std::rc::Rc;

/// Displays a popup to open a file and adds it to the primary text area ID (`tb`).
pub fn open_path_str(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(EditView::new().with_id("open_file_name").fixed_width(32))
            .title("Open file")
            .button("Open", |s| {
                let file_path = PathBuf::from(
                    Rc::try_unwrap(
                        s.call_on_id("open_file_name", |view: &mut EditView| view.get_content())
                            .unwrap(),
                    )
                    .unwrap(),
                ); // TODO fix

                if !file_path.exists() {
                    s.add_layer(Dialog::info(format!("{:?} does not exist", file_path)));
                    s.pop_layer();
                } else if file_path.is_dir() {
                    s.add_layer(Dialog::info(format!("{:?} is a directory", file_path)));
                    s.pop_layer();
                }

                s.call_on_id("tb", |view: &mut TextArea| {
                    view.set_content(get_path_content(&file_path))
                })
                .unwrap();
            })
            .button("Cancel", |s| {
                s.pop_layer();
            }),
    );
}

/// Gets content from a specified file path ([PathBuf]) and returns a string
/// or panics in the process.
pub fn get_path_content(path: &PathBuf) -> String {
    let mut got_file = File::open(path).unwrap();
    let mut content = String::new();

    got_file.read_to_string(&mut content).unwrap();

    content
}
