#!/bin/bash

VERSION=${1:-}

if [ -z ${VERSION} ]; then
  echo "Missing version for release";
  exit 1;
fi

git checkout v${VERSION}

cd ui
echo "Building web frontend ..."
yarn build

cd ../

rsync -av ui/build/static/ public/
cp ui/build/* public/

echo "Building application..."
cargo build --release

cp -r public target/release/
cd target
tar --transform "s,release,cuer_manager-$VERSION," -czf "cuer_manager-$VERSION.tar.gz" release/cuer_manager release/public

