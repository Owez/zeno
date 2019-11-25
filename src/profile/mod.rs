//! Contains items related to profiles & profile management. Please see [Profile]
//! for library-level profile management and [options]/[select] for cursive-based
//! popups related to profile management.

pub mod options;
pub mod select;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Storage structure for holding metadata for a given profile in-memory.
#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// Name of the profile
    pub name: String,

    /// Path to the theme file. This should be a valid theme toml or it will throw an error.
    pub theme: PathBuf,
}
