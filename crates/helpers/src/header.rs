use std::path::Path;

#[derive(Debug, Default)]
pub struct Header<'a> {
	functions: Vec<HeaderFunction<'a>>,
}

impl<'a> Header<'a> {
	pub fn from_file(path: &Path) -> Self {
		todo!()
	}

	pub fn push(&mut self, function: HeaderFunction<'a>) {
		self.functions.push(function);
	}

	pub fn to_le_bytes(&self) -> Vec<u8> {
		let mut result = vec![];
		for f in self.functions.iter() {
			f.to_le_bytes(&mut result);
		}
		result
	}
}

#[derive(Debug)]
pub struct HeaderFunction<'a> {
	name: &'a str,
	params: Vec<HeaderParam<'a>>,
}

impl<'a> HeaderFunction<'a> {
	pub fn new(name: &'a str) -> Self {
		Self { name, params: vec![], }
	}

	pub fn push(&mut self, param: HeaderParam<'a>) {
		self.params.push(param);
	}

	fn to_le_bytes(&self, buf: &mut Vec<u8>) {
		buf.extend(self.name.len().to_le_bytes());
		buf.extend(self.name.as_bytes());
		buf.extend(self.params.len().to_le_bytes());
		for p in self.params.iter() {
			p.to_le_bytes(buf);
		}
	}
}

#[derive(Debug)]
pub struct HeaderParam<'a> {
	key: &'a str,
	type_: HeaderType,
}

impl<'a> HeaderParam<'a> {
	pub fn new(key: &'a str, type_: HeaderType) -> Self {
		Self { key, type_ }
	}

	fn to_le_bytes(&self, buf: &mut Vec<u8>) {
		buf.extend(self.key.len().to_le_bytes());
		buf.extend(self.key.as_bytes());
		buf.push(self.type_.to_byte());
	}
}

#[derive(Debug)]
pub enum HeaderType {
	Bytes,
	// Native(ValType),
}

impl HeaderType {
	fn to_byte(&self) -> u8 {
		match self {
			Self::Bytes            => 0xfe,
			// Self::Native(val_type) => *val_type as u8,
		}
	}
}
