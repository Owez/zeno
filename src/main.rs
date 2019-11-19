use cursive::Cursive;
use std::env;
use std::path::PathBuf;

pub mod editor;
pub mod profile;
pub mod theme;

use profile::select::profile_select;

/// A structure for configuring the text editor before profile selecting.
///
/// This is useful for using in combination with a CLI to choose what to open
/// for example.
pub struct StartMeta {
    /// The path to automatically open after profile has been properly chosen.
    pub open_path: Option<PathBuf>,
}

/// Shows correct usage of zeno.
fn show_usage_info() {
    println!("Usage: `zeno [file]`");
    std::process::exit(0);
}

/// Retrives arguments from the command-line on startup. If these arguments are
/// incorrect, the function will display what went wrong and return a error exit
/// code.
pub fn get_cli_args() -> StartMeta {
    let open_path = env::args().nth(1);

    match open_path {
        None => StartMeta { open_path: None },
        Some(p) => {
            if p == "--help" || p == "-h" {
                show_usage_info()
            }

            let got_path = PathBuf::from(&p);

            if !got_path.exists() {
                println!(
                    "{:?} does not exist or zeno doesn't have the correct permissions to access it!\n",
                    got_path
                );
                show_usage_info()
            } else if got_path.is_dir() {
                println!("{:?} is a directory, not a file!\n", got_path);
                show_usage_info()
            }

            StartMeta {
                open_path: Some(got_path),
            }
        }
    }
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
    let args = get_cli_args(); // Parse arguments beforehand

    let mut siv = Cursive::default();

    siv.load_theme_file("data/themes/dark-mode.toml").unwrap(); // Dark default theme

    siv.add_global_callback('e', |s| s.quit());
    zeno_launch(&mut siv, args);
    siv.run();
}
