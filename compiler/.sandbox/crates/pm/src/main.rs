use reqwest::blocking::Client;
use structopt::StructOpt;

use std::error::Error;
use std::env::var;
use std::fs::read;

use dropin_utils::path::{get_build, get_root};

use dropin_pm::HOST;

#[derive(StructOpt, Debug)]
struct Cli {
  #[structopt()]
  owner: String,
  #[structopt()]
  model: String,
  #[structopt()]
  version: String,
}

fn main() -> Result<(), Box<dyn Error>> {
  let cli = Cli::from_args();

  let path = get_build(&get_root(), &cli.owner, &cli.model);
  if !path.exists() {
    panic!("Not found {}", path.to_str().unwrap());
  }

  let token = var("DROPIN_PM_TOKEN")?;

  let binary = read(&path)?;
  let client = Client::new();
  let url = format!(
    "{}/{}/{}/{}/{}", HOST, cli.owner, cli.model, cli.version, token,
  );
  let resp = client.post(&url).body(binary).send()?;
  if !resp.status().is_success() {
    panic!(
      "unexpected status from {} : {}",
      url,
      resp.status()
    );
  }
  Ok(())
}
