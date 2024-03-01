/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler
 *              |_|
 * Copyright © 2019-2024 Blue Forest
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

extern crate proc_macro;

use self::{rules::Rules, table::Table};
use abnf::rulelist;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::fs::read_to_string;
use syn::{parse_macro_input, ItemStruct, LitStr};

mod alternatives;
mod concatenation;
mod first;
mod follow;
mod production;
mod rules;
mod table;
mod term;
mod token;

#[proc_macro_attribute]
pub fn table(attr: TokenStream, item: TokenStream) -> TokenStream {
	let mut grammar: Option<LitStr> = None;
	let attr_parser = syn::meta::parser(|meta| {
		if meta.path.is_ident("grammar") {
			grammar = Some(meta.value()?.parse()?);
			Ok(())
		} else {
			Err(syn::Error::new(
				Span::call_site(),
				"grammar attribute is required",
			))
		}
	});
	parse_macro_input!(attr with attr_parser);
	let grammar = read_to_string(grammar.unwrap().value()).unwrap();
	let _table_struct = parse_macro_input!(item as ItemStruct);
	let rules = rulelist(&grammar).unwrap();
	let rules = Rules::new(rules);
	let table = Table::new(rules.iter());
	let non_terminals = &table.non_terminals;
	quote!(
		struct Table {
			non_terminals: std::collections::HashMap<u64, &'static str>,
		}

		impl Default for Table {
			fn default() -> Self {
				Self {
					non_terminals: std::collections::HashMap::from([
						#non_terminals
					]),
				}
			}
		}
	)
	.into()
}
