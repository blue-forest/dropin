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

if ! command -v rustfilt &> /dev/null; then
	cargo install rustfilt
fi

cd $(git rev-parse --show-toplevel)

OUT_DIR=$PWD/.coverage
IGNORE_LIST=".cargo|/rustc|.rustup|target"

eval "objects=($(cat $OUT_DIR/paths.txt))"

$(rustc +nightly --print target-libdir)/../bin/llvm-cov show \
  --instr-profile=$OUT_DIR/tests.profdata \
  --Xdemangler=rustfilt \
  --ignore-filename-regex=$IGNORE_LIST \
  --format html \
  --project-title "drop'in" \
  --output-dir=$OUT_DIR/report \
  --show-line-counts-or-regions \
  --show-instantiations \
  --use-color \
  $(for o in "${objects[@]}" target/debug/doctestbins/*/rust_out; do [[ -x $o ]] && printf "%s %s " --object $o; done)

echo "Report generated at $OUT_DIR/report/index.html"
