//! Theme module for retriving themes and allowing selection of them. This
//! integrates into the [cursive] crate and integrates a "theme-picker" that
//! loads themes for the given [crate::profile::Profile] when chosen.

use std::fs;
use std::path::PathBuf;

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

/// Boiler plate to get all themes from a given path. This is related to [Theme]
/// so will return [std::io::Error] directly if there is an error.
pub fn get_themes() -> Result<Vec<Theme>, std::io::Error> {
    let mut output = Vec::new();

    for path in fs::read_dir("data/themes")? {
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
