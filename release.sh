#!/bin/bash

VERSION="0.5.0"

cd ui
echo "Building web frontend ..."
yarn build

cd ../

rsync -av ui/build/static/ public/
cp ui/build/* public/

echo "Starting application..."
cargo build --release

cp -r public target/release/
cd target
tar --transform "s,release,cuer_manager-$VERSION," -czf "cuer_manager-$VERSION.tar.gz" release/cuer_manager release/public
