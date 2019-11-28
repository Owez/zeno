#!/bin/bash
echo "Installing.."

git clone https://gitlab.com/zeno-src/zeno
mv ./zeno/ zeno-build/
cd zeno-build/
cargo build --release
cd ..
mkdir zeno/
mv ./zeno-build/target/release/zeno ./zeno/
mv ./zeno-build/target/release/data ./zeno/
rm -rf zeno-build/
cd zeno/
strip ./zeno
chmod +x ./zeno

echo "zeno='.$PWD/zeno'" >> ~/.bash_aliases && source ~/.bash_aliases

echo "Installed @ './zeno/*' and in '~/.bash_alias'!"