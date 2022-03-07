/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| drop'in © 2019-2022 Blue Forest
 *              |_|
 * This code is free software distributed under GPLv3.
 */

use super::Type;

#[derive(Debug)]
pub struct Text {
}

impl Text {
  pub fn new() -> Self {
    Self{}
  }
}

impl Type for Text {}
