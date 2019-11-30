//! Theme module for retriving themes and allowing selection of them. This
//! integrates into the [cursive] crate and integrates a "theme-picker" that
//! loads themes for the given [crate::profile::Profile] when chosen.

use crate::profile::Profile;
use cursive::views::Dialog;
use cursive::Cursive;
use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;
use tinydb::Database;

/// Simple theme structure for storing an individual theme
#[derive(Debug, Clone)]
pub struct Theme {
    pub path: PathBuf,
    pub nickname: String,
}

impl Theme {
    /// Creates a [Theme] structure from a given path or returns [std::io::Error].
    pub fn from(path: PathBuf) -> Result<Self, std::io::Error> {
        Ok(Theme {
            path: path.clone(),
            nickname: path
                .file_stem()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap(),
        })
    }
}

/// Loads a theme into user and saves it to the [p_db]
pub fn load_theme(
    s: &mut Cursive,
    got_theme: &Theme,
    profile: Rc<RefCell<Profile>>,
    p_db: Rc<RefCell<Database<Profile>>>,
) {
    let mut p_db_mut = p_db.borrow_mut();
    let mut profile_mut = profile.borrow_mut();

    match p_db_mut.update_item(
        &profile_mut,
        Profile {
            name: profile_mut.name.clone(),
            theme: got_theme.path.clone(),
        },
    ) {
        Ok(_) => {
            profile_mut.update_theme(&got_theme); // update profile
            push_toml_theme(s, got_theme.path.clone());
        }
        Err(e) => s.add_layer(Dialog::info(format!(
            "Could not update theme! Error: '{:?}'",
            e
        ))),
    };
}

/// Loads toml file and pushes it to cursive. Please see [load_theme] (that uses
/// this function) if you'd like to push to database
fn push_toml_theme(s: &mut Cursive, path: PathBuf) {
    let open_file = |p: PathBuf| -> Result<String, std::io::Error> {
        let mut file = File::open(p)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    };

    match open_file(path) {
        Ok(c) => {
            match s.load_toml(&c) {
                Ok(_) => s.add_layer(Dialog::info("Loaded theme successfully!")),
                Err(_) => s.add_layer(Dialog::info(
                    "Failed to load theme, make sure it's a valid theme file!",
                )),
            };
        }
        Err(_) => s.add_layer(Dialog::info(
            "Could not load theme file, make sure it exists!",
        )),
    };
}

/// Boiler plate to get all themes from a given path. This is related to [Theme]
/// so will return [std::io::Error] directly if there is an error.
pub fn get_themes() -> Result<Vec<Theme>, std::io::Error> {
    let mut output = Vec::new();

    for path in std::fs::read_dir("data/themes")? {
        output.push(Theme::from(path?.path())?)
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_from_dark() {
        let nickname = String::from("dark-mode");

        let dark_theme = Theme::from(PathBuf::from(format!("data/themes/{}.toml", nickname)));
        assert_eq!(nickname, dark_theme.unwrap().nickname);
    }

    #[test]
    fn theme_from_light() {
        let nickname = String::from("light-mode");

        let light_theme = Theme::from(PathBuf::from(format!("data/themes/{}.toml", nickname)));
        assert_eq!(nickname, light_theme.unwrap().nickname);
    }
}
