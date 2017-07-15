#!/bin/bash

VERSION="0.1.0"

echo "Building application"
cargo build

cd static/js
echo "Building Ember app for release..."
node build/build.js

cd ../../

rsync -av static/js/dist/static/ public/
cp static/js/dist/index.html public/

./target/debug/cuer_manager
