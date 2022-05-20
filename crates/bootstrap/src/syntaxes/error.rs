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

use annotate_snippets::display_list::{DisplayList, FormatOptions};
use annotate_snippets::snippet::{
	Annotation, AnnotationType, Slice, Snippet, SourceAnnotation,
};

use dropin_helpers::PortableUnwrap;

use std::error::Error;
use std::fmt::{self, Display, Formatter};

macro_rules! pos {
	($module: ident, $iter: ident) => {
		*$iter.peek().map(|(i, _)| i).unwrap_or(&$module.len())
	}
}

macro_rules! err {
	($id: ident, $module: ident, $pos: expr, $($args:expr),*) => {
		Err(ParseError::new(
			$id, $module, $pos, format!($($args),*),
		))
	}
}

#[derive(Debug)]
pub struct ParseError<'a>{
	id: &'a str,
	module: &'a str,
	pos: usize,
	message: String,
}

impl<'a> ParseError<'a> {
	pub(super) fn new(
		id: &'a str, 
		module: &'a str, 
		pos: usize,
		message: String,
	) -> Self {
		Self { id, module, pos, message }
	}

	fn line_start(&self) -> (usize, usize) {
		let mut n_lines = 1;
		let mut line_start = 0;
		for (i, c) in self.module.get(..self.pos).punwrap().char_indices() {
			if c == '\n' {
				n_lines += 1;
				line_start = i;
			}
		}
		(n_lines, line_start)
	}
}

impl<'a> Display for ParseError<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
		let (n_lines, line_start) = self.line_start();
		let mut lines_end = 0;
		let mut lines_to_display = 3;
		for (i, c) in self.module.get(line_start..).punwrap().char_indices() {
			if c == '\n' {
				lines_end = i;
				if lines_to_display == 0 {
					break;
				}
				lines_to_display -= 1;
			}
		}
		let mut source = self.module.get(line_start..lines_end).punwrap();
		let pos = self.pos - line_start;
		let mut source_copy = String::new();
		let range = if pos == source.len() {
			source_copy.push_str(source);
			source_copy.push_str("<EOF>");
			source = &source_copy;
			(pos, pos+5)
		} else {
			(pos, pos+1)
		};

		let snippet = Snippet {
  		title: Some(Annotation {
    		label: Some(&self.message),
    		id: None,
    		annotation_type: AnnotationType::Error,
  		}),
  		footer: vec![],
  		slices: vec![
    		Slice {
      		source,
      		line_start: n_lines,
      		origin: Some(self.id),
      		fold: false,
      		annotations: vec![
      			SourceAnnotation {
      				label: "",
      				annotation_type: AnnotationType::Error,
      				range,
      			}
      		],
    		},
  		],
      opt: FormatOptions {
        color: true,
        ..Default::default()
      },
		};
		format!("{}", DisplayList::from(snippet)).fmt(f)
	}
}

impl<'a> Error for ParseError<'a> {}
