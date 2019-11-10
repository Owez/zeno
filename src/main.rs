#[macro_use]
extern crate clap;

use cursive::Cursive;
use std::path::PathBuf;

pub mod profile;
pub mod editor;
pub mod theme;

use profile::select::profile_select;

#[derive(Clap)]
#[clap(name = "zeno")]
struct Opt {
    #[clap(short = "o", long = "open")]
    file: Option<PathBuf>,
}

/// A structure for configuring the text editor before profile selecting.
///
/// This is useful for using in combination with a CLI to choose what to open
/// for example.
pub struct StartMeta {
    /// The path to automatically open after profile has been properly chosen.
    pub open_path: Option<PathBuf>,
}

/// Start of zeno's ui, enacting all basic functionality. You may pass in a file
/// to open automatically.
///
/// Internally, this is a modular boilerplate function for wrapping whatever
/// happens to start first.
pub fn zeno_launch(s: &mut Cursive, meta: StartMeta) {
    profile_select(s, meta);
}

fn main() {
    let opt = Opt::parse();
    let meta = StartMeta { open_path: opt.file };

    let mut siv = Cursive::default();

    siv.load_theme_file("data/themes/dark.toml").unwrap(); // Dark default theme

    siv.add_global_callback('e', |s| s.quit());
    zeno_launch(&mut siv, meta);
    siv.run();
}
