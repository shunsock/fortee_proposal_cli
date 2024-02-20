#!/bin/bash

function check_exit_status () {
  if [ $? -ne 0 ]; then
    echo "🚨 fail to install $1"
    echo "exitting..."
    exit 1
  else
    echo "✅ succeed to install $1"
  fi
}

cargo install cargo-edit
check_exit_status "cargo-edit"

cargo install cargo-watch
check_exit_status "cargo-watch"

rustup component add rustfmt
check_exit_status "rustfmt"

rustup component add clippy
check_exit_status "clippy"
