use bytes::Bytes;
use reqwest::Result;

pub const HOST: &str = "https://pm.dropin.dev.blueforest.cc";

pub fn fetch(owner: &str, model: &str, version: &str) -> Result<Bytes> {
	let mut url = String::from(HOST);
	url.push('/');
	url.push_str(owner);
	url.push('/');
	url.push_str(model);
	url.push('/');
	url.push_str(version);
	let resp = reqwest::blocking::get(&url)?;
	if !resp.status().is_success() {
		panic!("unexpected status from {} : {}", url, resp.status());
	}
	resp.bytes()
}
