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

GIT_HOOK_DIR=$(git rev-parse --show-toplevel)/.git/hooks

ETC_DIR=$(realpath $(dirname ${BASH_SOURCE[0]}))

if [ ! -f $GIT_HOOK_DIR/pre-commit ]; then
  echo "Adding Git pre-commit hook"
  ln -s $ETC_DIR/hooks/pre-commit.sh $GIT_HOOK_DIR/pre-commit
fi
