use crate::profile::Profile;
use std::cell::RefCell;
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
