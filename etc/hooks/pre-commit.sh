#!/bin/bash

set -e

ROOT_DIR=$(realpath $(dirname ${BASH_SOURCE[0]}))/../..

SRC_DIR=$ROOT_DIR/src

COPYING_HEADER="\/*     _              _ _\n *  __| |_ _ ___ _ __( |_)_ _\n * \/ _\` | '_\/ _ \\\\ '_ \\\\\/| | ' \\\\\n * \\\\__,_|_| \\\\___\/ .__\/ |_|_||_| drop'in © 2019-2022 Blue Forest\n *              |_|\n * This code is free software distributed under GPLv3.\n *\/\n\n"

for i in $(find $SRC_DIR -name '*.rs' -or -name '*.pest'); do
  if ! grep -q "This code is free software distributed under GPLv3" $i; then
    echo "Adding license header to $i"
    sed -e "1s/^/$COPYING_HEADER/" $i > $i
  fi
done
