#!/bin/bash

VERSION="0.1.0"
ENV=development

mkdir -p public

cd static/js
echo "Building Ember app ..."
node build/build.js

cd ../../

rsync -av static/js/dist/static/ public/
cp static/js/dist/index.html public/

echo "Starting application..."
cargo run
