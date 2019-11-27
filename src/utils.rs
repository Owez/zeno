//! Shared functions for backend of Zeno. See the common [find_profile] as an
//! example of this.

use crate::profile::Profile;
use std::cell::RefCell;
use std::env::current_exe;
use std::io;
use std::path::PathBuf;
use std::rc::Rc;
use tinydb::Database;

/// Searches a database for a profile given it's name. If it exists, the function
/// will return it's details. If not, a panic will be raised.
pub fn find_profile(p_db: Rc<RefCell<Database<Profile>>>, search_name: &str) -> Profile {
    p_db.borrow()
        .query_item(|q: &Profile| &q.name, String::from(search_name))
        .unwrap()
        .clone()
}

/// Prefixes the given path with the currently executing directory
pub fn dir_append(path: PathBuf) -> Result<PathBuf, io::Error> {
    let mut cur_dir = current_exe()?;
    cur_dir.pop(); // remove exe file
    cur_dir.push(path);

    Ok(cur_dir)
}
