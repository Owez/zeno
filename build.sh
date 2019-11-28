#!/bin/bash

DIR="$(dirname "$0")"

if [ -d "$DIR/target/debug" ]; then
    cp -r $DIR/data $DIR/target/debug/data
fi

if [ -d "$DIR/target/release" ]; then
    cp -r $DIR/data $DIR/target/release/data
fi

cargo $@