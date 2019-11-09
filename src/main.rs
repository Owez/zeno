#[macro_use]
extern crate clap;

use cursive::Cursive;
use std::path::PathBuf;

mod lib;

use lib::*;

#[derive(Clap)]
#[clap(name = "zeno")]
struct Opt {
    #[clap(short = "f", long = "file")]
    file: Option<PathBuf>,
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
