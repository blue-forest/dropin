/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
 *              |_|
 * Copyright © 2019-2022 Blue Forest
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

pub const FD_DATASYNC:             u64 = 1 <<  0;
pub const FD_READ:                 u64 = 1 <<  1;
pub const FD_SEEK:                 u64 = 1 <<  2;
pub const FD_FDSTAT_SET_FLAGS:     u64 = 1 <<  3;
pub const FD_SYNC:                 u64 = 1 <<  4;
pub const FD_TELL:                 u64 = 1 <<  5;
pub const FD_WRITE:                u64 = 1 <<  6;
pub const FD_ADVISE:               u64 = 1 <<  7;
pub const FD_ALLOCATE:             u64 = 1 <<  8;
pub const PATH_CREATE_DIRECTORY:   u64 = 1 <<  9;
pub const PATH_CREATE_FILE:        u64 = 1 << 10;
pub const PATH_LINK_SOURCE:        u64 = 1 << 11;
pub const PATH_LINK_TARGET:        u64 = 1 << 12;
pub const PATH_OPEN:               u64 = 1 << 13;
pub const FD_READDIR:              u64 = 1 << 14;
pub const PATH_READLINK:           u64 = 1 << 15;
pub const PATH_RENAME_SOURCE:      u64 = 1 << 16;
pub const PATH_RENAME_TARGET:      u64 = 1 << 17;
pub const PATH_FILESTAT_GET:       u64 = 1 << 18;
pub const PATH_FILESTAT_SET_SIZE:  u64 = 1 << 19;
pub const PATH_FILESTAT_SET_TIMES: u64 = 1 << 20;
pub const FD_FILESTAT_GET:         u64 = 1 << 21;
pub const FD_FILESTAT_SET_SIZE:    u64 = 1 << 22;
pub const FD_FILESTAT_SET_TIMES:   u64 = 1 << 23;
pub const PATH_SYMLINK:            u64 = 1 << 24;
pub const PATH_REMOVE_DIRECTORY:   u64 = 1 << 25;
pub const PATH_UNLINK_FILE:        u64 = 1 << 26;
pub const POLL_FD_READWRITE:       u64 = 1 << 27;
pub const SOCK_SHUTDOWN:           u64 = 1 << 28;
pub const SOCK_ACCEPT:             u64 = 1 << 29;
