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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 * 
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use home::home_dir;
use path_clean::PathClean;
use yaml_rust::YamlLocader;

use std::env::{current_dir, var};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::{create_dir, File, read_to_string};
use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

pub fn setup_config(username: Option<String>) {
  let root = get_root();
  let username_str = username.unwrap_or_else(get_username);
  println!("{}", root.to_str().unwrap());
  if !root.exists() {
    create_dir(&root).unwrap();
    let mut file = File::create(root.join("config.yml")).unwrap();
    file.write_all(format!("username: {}", username_str).as_bytes()).unwrap();
    println!("Config file created");
  } else {
    println!("Root already exists");
    read_to_string(root.join("config.yml")).unwrap()
  }
}

fn get_username() -> String {
  loop {
    let mut input = String::new();
    print!("How should you be called ?");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    break input.trim_end_matches('\n').to_string();
  }
}

fn get_root() -> PathBuf {
  if let Ok(root) = var("DROPIN_ROOT") {
    println!("Using $DROPIN_ROOT ({})", root);
    return PathBuf::from(root);
  }
  loop {
    let mut input = String::new();
    print!("Where should drop'in file be stored ? (~/dropin) ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    let mut result = PathBuf::from(input.trim_end_matches('\n'));
    if input == "\n" {
      match home_dir() {
        Some(path) => {
          result = path;
          result.push("dropin");
        }
        None => {
          println!("Can't find any HOME dir !");
          continue;
        }
      }
    } else if input == "" { // EOF
      exit(0);
    }
    if result.is_relative() {
      result = current_dir().unwrap().join(result).clean();
    }
    if let Err(err) = validate_root(&result) {
      println!("{}", err);
      continue;
    }
    
    break result;
  }
}

fn validate_root(path: &PathBuf) -> Result<(), ConfigError> {
  if !path.exists() {
    match path.parent() {
      Some(parent) => { check_permissions(parent, true) }
      None => {
        Err(ConfigError::from("Can't find the parent directory"))
      }
    }
  } else {
    let config = path.join("config.yml");
    if !config.exists() {
      Err(ConfigError::new(format!(
        "Path \"{}\" exists and does not contain a valid configuration file",
        path.to_str().unwrap_or("[non-utf8]"),
      )))
    } else {
      check_permissions(&config, false)
    }
  }
}

fn check_permissions(path: &Path, is_dir: bool) -> Result<(), ConfigError> {
  match path.metadata() {
    Ok(metadata) => {
      if metadata.is_dir() != is_dir {
        return Err(ConfigError::new(format!(
          "\"{}\" is a {}",
          path.to_str().unwrap_or("[non-utf8]"),
          if is_dir { "file" } else { "directory" },
        )));
      }
      if metadata.permissions().readonly() {
        return Err(ConfigError::new(format!(
          "You cannot write into \"{}\"",
          path.to_str().unwrap_or("[non-utf8]"),
        )));
      }
      Ok(())
    }
    Err(err)     => {
      Err(ConfigError::new(format!("{}", err)))
    }
  }
}

#[derive(Debug)]
pub struct ConfigError(String);

impl ConfigError {
  pub fn new(message: String) -> Self { Self(message) }
}

impl From<&str> for ConfigError {
  fn from(message: &str) -> Self { Self(message.to_string()) }
}

impl Display for ConfigError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    self.0.fmt(f)
  }
}

impl Error for ConfigError {}
