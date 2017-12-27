#!/bin/sh

set -e

if [ $# != 1 ]; then
  echo "Usage: $(basename $0) version" >&2
  exit 1
fi
version="$1"

shasum_cmd="sha256sum"

if [ "$(uname)" = "Darwin" ]; then
  shasum_cmd="shasum -a 256"
fi

# Linux and Darwin builds.
for arch in i686 x86_64; do
  for target in apple-darwin unknown-linux-musl unknown-linux-gnu; do
    url="https://github.com/trevershick/djs/releases/download/$version/djs-$version-$arch-$target.tar.gz"
    if curl --output /dev/null --silent --head --fail "$url"; then
      sha=$(curl -sfSL "$url" | $shasum_cmd)
      echo "$version-$arch-$target $sha"
    fi
  done
done

# Source.
for ext in zip tar.gz; do
  url="https://github.com/trevershick/djs/archive/$version.$ext"
  if curl --output /dev/null --silent --head --fail "$url"; then
    sha=$(curl -sfSL "$url" | $shasum_cmd)
    echo "source.$ext $sha"
  fi
done
