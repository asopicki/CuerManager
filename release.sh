#!/bin/bash

VERSION=${1:-}

if [ -z ${VERSION} ]; then
  echo "Missing version for release";
  exit 1;
fi

git checkout v${VERSION}

cd ui
echo "Building web frontend ..."
ng build --aot=false --prod --deploy-url=static/

cd ../

rsync -av ui/dist/cuer-manager-ui public/
#cp ui/build/* public/

echo "Building application..."
cargo build --release

cp -r public target/release/
cd target
tar --transform "s,release,cuer_manager-$VERSION," -czf "cuer_manager-$VERSION.tar.gz" release/cuer_manager release/public

