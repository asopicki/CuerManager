#!/bin/bash

VERSION=${1:-}

if [ -z ${VERSION} ]; then
  echo "Missing version for release";
  exit 1;
fi

if [ ${VERSION} != "HEAD" ]; then
  echo "Versioned release not supported"
  exit 1;
  git checkout v${VERSION}
else
  VERSION=`date -u +%s`
fi

cd cuer_manager_backend/ui
echo "Building web frontend ..."
ng build --prod --deploy-url=static/

cd ../../

if [ -d public ]; then
  rm -r public
fi

cp -r cuer_manager_backend/ui/dist/cuer-manager-ui/ public

echo "Building application..."
cargo build --release

if [ -d target/release/public ]; then
  rm -r target/release/public
fi

cp -r public target/release/

cd target
tar --transform "s,release,cuer_manager-$VERSION," -czf "cuer_manager-$VERSION.tar.gz" release/cuecard_indexer release/cuer_manager release/public

