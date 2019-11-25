//! Theme module for retriving themes and allowing selection of them. This
//! integrates into the [cursive] crate and integrates a "theme-picker" that
//! loads themes for the given [crate::profile::Profile] when chosen.

mod get_themes;

pub use get_themes::*;
