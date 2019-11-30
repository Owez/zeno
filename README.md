# Zeno

Zeno is a new code editor with ease-of-use in mind.

**NOTE: This repository is still under heavy development and is currently not anywhere near a production build.**

## Installing

### Linux

- Master git branch:

  ```bash
  wget https://gitlab.com/zeno-src/zeno/raw/master/scripts/install_linux.sh && sh -e ./install_linux.sh
  ```

- Recent Linux x86 binaries: [here](https://gitlab.com/zeno-src/zeno/pipelines?scope=branches&page=1/)

### Windows & OSX

Windows & OSX is not officially supported but *should* work as [crossterm](https://lib.rs/crates/crossterm) is cross-platform (the backend used for creating the terminal/text interface). Therefore to run it on windows, you need to manually build Zeno.

Please install [Rust](https://www.rust-lang.org/tools/install) & [git](https://git-scm.com/download/win) then follow the steps below:

1. Open git bash
2. Clone the Zeno editor:

   ```bash
   git clone https://gitlab.com/zeno-src/zeno
   ```

3. Change into the newly-created `zeno/` directory:

   ```bash
   cd zeno
   ```

4. Build Zeno:

   ```bash
   cargo build --release
   ```

5. Get `zeno.exe` from `target/release/` and move it into a new folder along with the `data/` directory, stored next to this.

## Development Guide

### Debugging

Zeno includes the `/data` directory (as you can find next to this README). This contains themes and the location of the saved database.

When building zeno for debugging purposes (e.g. `cargo run`), you need to also include the current root `/data` in the build/target directory. To do this, you can use the `./build.sh` script that replaces the `cargo` prefix.

This file automatically includes the data directory, depending on what you're running. *If it doesn't work, please try again as it detects the files before running cargo*.

For example:

- `cargo run` -> `./build.sh run`
- `cargo build` -> `./build.sh build`
