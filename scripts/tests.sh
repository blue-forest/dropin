#!/bin/bash

#     _              _ _
#  __| |_ _ ___ _ __( |_)_ _
# / _` | '_/ _ \ '_ \/| | ' \
# \__,_|_| \___/ .__/ |_|_||_| dropin-compiler
#              |_|
# Copyright © 2019-2024 Blue Forest
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published
# by the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

set -e

if ! rustup component list | grep -q "llvm-tools-"; then
	rustup toolchain install nightly --component llvm-tools
fi

if ! command -v jq &> /dev/null; then
	echo "jq is required to run this script"
	exit 1
fi

cd $(git rev-parse --show-toplevel)

OUT_DIR=$PWD/.coverage
rm -rf $OUT_DIR
mkdir -p $OUT_DIR

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-C instrument-coverage -Z profile -C codegen-units=1 -C opt-level=0 -C link-dead-code -C overflow-checks=off -Z panic_abort_tests"
export RUSTDOCFLAGS="-C instrument-coverage -Z unstable-options --persist-doctests target/debug/doctestbins"
export LLVM_PROFILE_FILE=$OUT_DIR/%p-%m.profraw

cargo +nightly test --message-format=json --verbose | grep "{" | grep "}" |
  jq -r "select(.profile.test == true) | .filenames[]" |
  grep -v dSYM - >$OUT_DIR/paths.txt

cd $OUT_DIR

$(rustc +nightly --print target-libdir)/../bin/llvm-profdata merge \
	$(find . -name "*.profraw") --output tests.profdata

rm -rf *.profraw
