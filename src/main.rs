use cursive::Cursive;

mod lib;
use lib::*;

fn main() {
    let mut siv = Cursive::default();

    siv.add_global_callback('e', |s| s.quit());
    zeno_launch(&mut siv);
    siv.run();
}
