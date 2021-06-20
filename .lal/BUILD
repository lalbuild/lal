#!/bin/bash
set -e

export PKG_CONFIG_ALLOW_CROSS=1

main() {
  if [ ! -d ~/.cargo/registry ]; then
    echo "Ensure you have created a cargo-cache docker volume to speed up subsequent builds"
    echo "If this is your first build, this is normal"
    echo "Otherwise, please 'docker volume create cargo-cache' and ensure it is specified in your lal config"
    echo "Continuing from blank cache..."
  fi
  if [[ $1 == "lal" ]]; then
    mkdir -p OUTPUT/{bin,share/lal}
    cp lal.complete* OUTPUT/share/lal/
    if [[ $2 == "slim" ]]; then
      (
        set -x; cargo build \
        --target x86_64-unknown-linux-musl \
        --no-default-features \
        --release \
        --verbose
      )
      cp ./target/x86_64-unknown-linux-musl/release/lal OUTPUT/bin/
    elif [[ $2 == "release" ]]; then
      (
        set -x; cargo build \
        --target x86_64-unknown-linux-musl \
        --release \
        --verbose
      )
      cp ./target/x86_64-unknown-linux-musl/release/lal OUTPUT/bin/
    elif [[ $2 == "debug" ]]; then
      (
        set -x; cargo build \
        --target x86_64-unknown-linux-musl \
        --verbose
      )
      cp ./target/x86_64-unknown-linux-musl/debug/lal OUTPUT/bin/
    else
      echo "No such configuration $2 found"
      exit 2
    fi
  elif [[ $1 == "lal-unit-tests" ]]; then
    cargo build --test testmain
    cp ./target/x86_64-unknown-linux-musl/debug/testmain-* OUTPUT/
    rm -f OUTPUT/testmain-*.d
    echo "Please run the testmain executable in ./OUTPUT/"
  else
    echo "No such component $1 found"
    exit 2
  fi
}

# If we were not sourced as a library, pass arguments onto main
if [ "$0" = "${BASH_SOURCE[0]}" ]; then
  main "$@"
else
  echo "${BASH_SOURCE[0]} sourced"
fi
