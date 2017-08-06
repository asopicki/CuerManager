#!/bin/bash

VERSION="0.1.0"

mkdir -p public

cd ui
echo "Building web frontend ..."
yarn build

cd ../

rsync -av ui/build/static/ public/
cp ui/build/* public/

echo "Starting application..."
cargo run
