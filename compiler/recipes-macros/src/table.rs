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

use crate::{first::First, follow::Follow, rules::Rule};
use dropin_compiler_common::TokenKind;
use quote::{quote, ToTokens};
use serde::{
  ser::{SerializeMap, SerializeStruct},
  Serialize, Serializer,
};
use std::{
  collections::{HashMap, HashSet},
  ops::{Deref, DerefMut},
};

pub struct Table<'a> {
  pub(crate) non_terminals: NonTerminals<'a>,
  productions: Vec<(&'a str, Vec<TokenKind<'a>>)>,
  data: TableData<'a>,
}

impl<'a> ToTokens for Table<'a> {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let productions = self.productions.iter().fold(
      proc_macro2::TokenStream::new(),
      |mut stream, (_, tokens)| {
        let capacity = tokens.len();
        stream.extend(
          quote!(let mut rules = std::vec::Vec::with_capacity(#capacity);),
        );
        for token in tokens.into_iter().rev() {
          stream.extend(quote!(rules.push(#token);));
        }
        stream.extend(quote!(productions.push(rules);));
        stream
      },
    );
    let mut data = proc_macro2::TokenStream::new();
    for (non_terminal, _) in self.non_terminals.iter() {
      data
        .extend(quote!(let mut redirect = std::collections::BTreeMap::new();));
      for (token, production) in self.data[non_terminal].iter() {
        data.extend(quote!(redirect.insert(#token, #production);));
      }
      data.extend(quote!(data.insert(#non_terminal, redirect);));
    }

    let productions_capacity = self.productions.len();

    tokens.extend(quote!(
			pub struct Table {
				pub productions: std::vec::Vec<
                  std::vec::Vec<
                    dropin_compiler_common::TokenKind<'static>
                  >
                >,
				pub data: std::collections::BTreeMap<
                  &'static str,
                  std::collections::BTreeMap<
                    dropin_compiler_common::TokenKind<'static>,
                    usize
                  >
                >
			}

			impl Default for Table {
				fn default() -> Self {
					let mut productions = std::vec::Vec::with_capacity(#productions_capacity);
					#productions
					let mut data = std::collections::BTreeMap::new();
					#data
					Self { productions, data }
				}
			}
		))
  }
}

impl<'a> Table<'a> {
  pub fn new(mut rules: impl Iterator<Item = Rule<'a>>) -> Self {
    let mut first = First::default();
    let mut follow = Follow::default();
    let mut non_terminals = HashMap::new();
    let mut non_terminal_id = 0;
    let mut productions: Vec<(&'a str, Vec<TokenKind<'a>>)> = vec![];
    let mut is_first = true;
    while let Some(rule) = rules.next() {
      let name = rule.name();
      first.insert_non_terminal(name);
      follow.insert_non_terminal(name, is_first);
      non_terminals.entry(name).or_insert_with(|| {
        let id = non_terminal_id;
        non_terminal_id += 1;
        id
      });
      is_first = false;
      for tokens in rule.iter() {
        for token in tokens.iter() {
          match token {
            TokenKind::NonTerminal(_) => {}
            _ => first.insert_terminal(*token),
          }
        }
        productions.push((name, tokens));
      }
    }

    first.init(&productions);
    follow.init(&first, &productions);

    let mut data: TableData<'a> = HashMap::new();
    for (i, (name, tokens)) in productions.iter().enumerate() {
      let mut all_first = HashSet::new();
      let mut is_empty = true;
      for token in tokens {
        let token_first = first.get(token);
        all_first.extend(token_first);
        if !token_first.contains(&TokenKind::Empty) {
          is_empty = false;
          break;
        }
      }
      if is_empty {
        all_first.extend(follow.get(name));
      }
      for token in all_first {
        if let TokenKind::Empty = token {
          continue;
        }
        if let Some(production) =
          data.entry(name).or_insert(HashMap::new()).insert(token, i)
        {
          let (collision_name, collision_tokens) = &productions[production];
          eprintln!(
            "{collision_name} = {}",
            collision_tokens
              .iter()
              .map(|token| { format!("{token:?}") })
              .collect::<Vec<_>>()
              .join(" ")
          );
          eprintln!(
            "{name} = {}",
            tokens
              .iter()
              .map(|token| { format!("{token:?}") })
              .collect::<Vec<_>>()
              .join(" ")
          );
          panic!("COLLISION ON {token:?}");
        };
      }
    }
    Self {
      non_terminals: NonTerminals(non_terminals),
      productions,
      data,
    }
  }
}

impl<'a> Serialize for Table<'a> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut table = serializer.serialize_struct("Table", 3)?;
    //table.serialize_field("non_terminals", &self.non_terminals)?;
    // table.serialize_field(
    // 	"productions",
    // 	&SerializableProductions {
    // 		productions: &self.productions,
    // 		non_terminals: &self.non_terminals,
    // 	},
    // )?;
    table.serialize_field(
      "data",
      &SerializableTableData {
        data: &self.data,
        non_terminals: &self.non_terminals,
      },
    )?;
    table.end()
  }
}

pub(crate) struct NonTerminals<'a>(HashMap<&'a str, u64>);

impl<'a> Deref for NonTerminals<'a> {
  type Target = HashMap<&'a str, u64>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'a> DerefMut for NonTerminals<'a> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

/*
impl<'a> Serialize for NonTerminals<'a> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.collect_map(self.iter().map(|(k, v)| (v, k)))
  }
}

struct SerializableProductions<'rules, 'table> {
  productions: &'table Vec<(&'rules str, Vec<TokenKind<'rules>>)>,
}

impl<'rules, 'table> ToTokens for SerializableProductions<'rules, 'table> {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    todo!()
  }
}

impl<'rules, 'table> Serialize for SerializableProductions<'rules, 'table> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut seq = serializer.serialize_seq(Some(self.productions.len()))?;
    for (_, tokens) in self.productions.iter() {
      seq.serialize_element(&SerializableTokens {
        tokens,
        non_terminals: &self.non_terminals,
      })?;
    }
    seq.end()
  }
}
*/

impl<'a> ToTokens for NonTerminals<'a> {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    for (k, v) in self.iter() {
      tokens.extend(quote!((#v, #k),))
    }
  }
}

/*
struct SerializableProduction<'rules, 'table> {
  production: &'table (&'rules str, Vec<TokenKind<'rules>>),
  non_terminals: &'table HashMap<&'rules str, u64>,
}

impl<'rules, 'table> Serialize for SerializableProduction<'rules, 'table> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut tuple = serializer.serialize_tuple(2)?;
    tuple
      .serialize_element(self.non_terminals.get(self.production.0).unwrap())?;
    tuple.serialize_element(&SerializableTokens {
      tokens: &self.production.1,
      non_terminals: &self.non_terminals,
    })?;
    tuple.end()
  }
}

struct SerializableTokens<'rules, 'table> {
  tokens: &'table Vec<TokenKind<'rules>>,
  non_terminals: &'table HashMap<&'rules str, u64>,
}

impl<'rules, 'table> Serialize for SerializableTokens<'rules, 'table> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut seq = serializer.serialize_seq(Some(self.tokens.len()))?;
    for token in self.tokens.iter().rev() {
      if let TokenKind::NonTerminal(name) = token {
        seq.serialize_element(&self.non_terminals.get(name).unwrap())?;
      } else {
        seq.serialize_element(token.as_str())?;
      }
    }
    seq.end()
  }
}
*/

/*impl ToTokens for SerializableTokens<'_, '_> {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let mut content = proc_macro2::TokenStream::new();
    for token in self.tokens.iter().rev() {
      if let TokenKind::NonTerminal(name) = token {
        content.extend(quote!(#name,))
      } else {
        content.extend(quote!(#token,))
      }
    }
    tokens.extend(quote!(vec![#content]))
  }
}*/

type TableData<'a> = HashMap<&'a str, HashMap<TokenKind<'a>, usize>>;

struct SerializableTableData<'rules, 'table> {
  data: &'table TableData<'rules>,
  non_terminals: &'table HashMap<&'rules str, u64>,
}

impl<'rules, 'table> Serialize for SerializableTableData<'rules, 'table> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut map = serializer.serialize_map(Some(self.data.len()))?;
    for (name, token_mapping) in self.data.iter() {
      map.serialize_entry(
        self.non_terminals.get(name).unwrap(),
        &SerializableTokenMapping(token_mapping),
      )?;
    }
    map.end()
  }
}

struct SerializableTokenMapping<'rules, 'table>(
  &'table HashMap<TokenKind<'rules>, usize>,
);

impl<'rules, 'table> Serialize for SerializableTokenMapping<'rules, 'table> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut map = serializer.serialize_map(Some(self.0.len()))?;
    for (token, production) in self.0.iter() {
      map.serialize_entry(token.as_str(), production)?;
    }
    map.end()
  }
}
