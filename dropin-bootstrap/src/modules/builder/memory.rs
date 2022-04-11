use wasm_encoder::{DataSection, Instruction, ValType};

pub struct MemoryBuilder<'a> {
  data:       Vec<&'a [u8]>,
  data_len:   usize,
  buffer_len: usize,
}

impl<'a> Default for MemoryBuilder<'a> {
  fn default() -> Self {
    Self{
      data:       vec![],
      data_len:   0,
      buffer_len: 0,
    }
  }
}

impl<'a> MemoryBuilder<'a> {
  pub fn data(&mut self, data: &'a [u8]) -> MemoryAddress {
    let result = MemoryAddress::Data(self.data_len as u32);
    self.data_len += data.len();
    self.data.push(data);
    result
  }

  pub fn buffer(&mut self, valtype: ValType) -> MemoryAddress {
    let result = MemoryAddress::Buffer(self.buffer_len as u32);
    self.buffer_len += match valtype {
      ValType::I32       =>  4,
      ValType::I64       =>  8,
      ValType::F32       =>  4,
      ValType::F64       =>  8,
      ValType::V128      => 16,
      ValType::FuncRef   =>  4,
      ValType::ExternRef =>  4,
    };
    result
  }

  pub fn resolve_addr(&self, addr: &MemoryAddress) -> u32 {
    let mut result = 16;
    if let MemoryAddress::Data(offset) = addr {
      return result + offset;
    }
    result += (self.data_len + 4 - (self.data_len % 4)) as u32;
    if let MemoryAddress::Buffer(offset) = addr {
      return result + offset;
    }
    unreachable!()
  }

  pub fn build_data(&self) -> Option<DataSection> {
    if self.data.is_empty() {
      return None;
    }
    let mut offset = 16;
    let mut result = DataSection::new();
    for d in self.data.iter() {
      result.active(
        0, &Instruction::I32Const(offset as i32), d.iter().copied(),
      );
      offset += d.len();
    }
    Some(result)
  }
}

pub enum MemoryAddress {
  Data(u32),
  Buffer(u32),
}
