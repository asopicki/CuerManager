#!/bin/bash

VERSION="0.3.0"

echo "Building application for release..."
cargo build --release

cd static/js
echo "Building Ember app for release..."
node build/build.js

cd ../../

rsync -av static/js/dist/static/ public/
cp static/js/dist/index.html public/

cp -r public target/release/
cd target
tar --transform "s,release,cuer_manager-$VERSION," -czf "cuer_manager-$VERSION.tar.gz" release/cuer_manager release/public
