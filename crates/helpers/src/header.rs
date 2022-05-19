use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::slice::Iter;
use std::str;

macro_rules! try_ {
	($expr:expr $(,)?) => {
		$expr.map_err(|_| InvalidHeader {})?
	};
}

macro_rules! opt {
	($expr:expr $(,)?) => {
		match $expr {
			Some(content) => content,
			None => return Err(InvalidHeader {}),
		}
	};
}

#[derive(Debug)]
pub struct InvalidHeader;

impl Display for InvalidHeader {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		"Invalid header file. Please compile or download again".fmt(f)
	}
}

impl Error for InvalidHeader {}

#[derive(Debug, Default)]
pub struct Header<'a> {
	functions: Vec<HeaderFunction<'a>>,
}

impl<'a> Header<'a> {
	pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, InvalidHeader> {
		let mut result = Self::default();
		let mut offset = 0;
		while offset < bytes.len() {
			let (function, size) =
				try_!(HeaderFunction::from_bytes(opt!(bytes.get(offset..)),));
			result.push(function);
			offset += size;
		}
		Ok(result)
	}

	fn read_u32(bytes: &[u8]) -> u32 {
		let mut result_bytes: [u8; 4] = Default::default();
		result_bytes.copy_from_slice(&bytes[0..4]);
		u32::from_le_bytes(result_bytes)
	}

	pub fn push(&mut self, function: HeaderFunction<'a>) {
		self.functions.push(function);
	}

	pub fn functions(&self) -> Iter<HeaderFunction<'a>> {
		self.functions.iter()
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
	pub fn from_bytes(bytes: &'a [u8]) -> Result<(Self, usize), InvalidHeader> {
		let name_len = Header::read_u32(bytes);
		let mut offset = (name_len as usize) + 4;
		let name = try_!(str::from_utf8(opt!(bytes.get(4..offset))));
		let mut result = HeaderFunction::new(name);
		let n_params = Header::read_u32(opt!(bytes.get(offset..)));
		offset += 4;
		for _ in 0..n_params {
			let (param, size) =
				try_!(HeaderParam::from_bytes(opt!(bytes.get(offset..)),));
			result.push(param);
			offset += size;
		}

		Ok((result, offset))
	}

	pub fn new(name: &'a str) -> Self {
		Self {
			name,
			params: vec![],
		}
	}

	pub fn push(&mut self, param: HeaderParam<'a>) {
		self.params.push(param);
	}

	pub fn name(&self) -> &str {
		self.name
	}

	pub fn params(&self) -> Iter<HeaderParam<'a>> {
		self.params.iter()
	}

	fn to_le_bytes(&self, buf: &mut Vec<u8>) {
		buf.extend((self.name.len() as u32).to_le_bytes());
		buf.extend(self.name.as_bytes());
		buf.extend((self.params.len() as u32).to_le_bytes());
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
	pub fn from_bytes(bytes: &'a [u8]) -> Result<(Self, usize), InvalidHeader> {
		let key_len = Header::read_u32(bytes);
		let offset = (key_len as usize) + 4;
		let key = try_!(str::from_utf8(opt!(bytes.get(4..offset))));
		let type_ = try_!(HeaderType::from_byte(*opt!(bytes.get(offset))));
		let result = Self::new(key, type_);
		Ok((result, offset + 1))
	}

	pub fn new(key: &'a str, type_: HeaderType) -> Self {
		Self { key, type_ }
	}

	pub fn key(&self) -> &str {
		self.key
	}

	pub fn type_(&self) -> &HeaderType {
		&self.type_
	}

	fn to_le_bytes(&self, buf: &mut Vec<u8>) {
		buf.extend((self.key.len() as u32).to_le_bytes());
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
			Self::Bytes => 0xfe,
			// Self::Native(val_type) => *val_type as u8,
		}
	}

	fn from_byte(byte: u8) -> Result<Self, InvalidHeader> {
		match byte {
			0xfe => Ok(Self::Bytes),
			_ => Err(InvalidHeader {}),
		}
	}
}

impl Display for HeaderType {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		match self {
			Self::Bytes => "bytes",
			// Self::Native(val_type) => *val_type as u8,
		}
		.fmt(f)
	}
}
