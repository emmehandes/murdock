use std::io::{Error, ErrorKind};
use std::cell::Cell;

pub enum Limits {
  Size = 512,
  LabelLength = 0x34,
}

pub struct Writer<'a> {
  buf: &'a mut [u8],
  pos: Cell<usize>,
}

pub struct Reader<'a> {
  buf: &'a [u8],
  pos: Cell<usize>,
}

impl<'a> Writer<'a> {
  pub fn new(dst: &'a mut[u8]) -> Writer <'a> {
    Writer {buf: dst, pos: Cell::new(0) }
  }

  pub fn pos(&self) -> usize { self.pos.get() }

  pub fn write_u8(&mut self, val: u8) -> Result<(), Error> {
    if self.pos.get() >= self.buf.len() {
      return Err(Error::new(ErrorKind::InvalidInput, "End of buffer write"));
    }
    self.buf[self.pos.get()] = val;
    self.pos.set(self.pos.get() + 1);
    Ok(())
  }

  pub fn write_u16(&mut self, val: u16) -> Result<(), Error> {
    self.write_u8((val>>8) as u8)?;
    self.write_u8((val & 0xFF) as u8)?;
    Ok(())
  }

  pub fn write_u32(&mut self, val: u32) -> Result<(), Error> {
    self.write_u16((val>>16) as u16)?;
    self.write_u16((val & 0xFFFF) as u16)?;
    Ok(())
  }

  pub fn write_name(&mut self, name: &str) -> Result<(),Error> {
    let split_name = name.split('.').collect::<Vec<&str>>();

    for label in split_name {
      let label_len = label.len();
      if label_len > Limits::LabelLength as usize {
        return Err(Error::new(ErrorKind::InvalidInput, "Single label exceeds 63 characters"));
      }
      else {
        self.write_u8(label_len as u8)?;
        for byte in label.as_bytes() {
          self.write_u8(*byte)?;
        }
      }
    }
    self.write_u8(0)?;
    Ok (())
  }
}

impl<'a> Reader<'a> {
  pub fn new(src: &'a [u8]) -> Reader <'a> {
    Reader {buf: src, pos: Cell::new(0) }
  }

  pub fn pos(&self) -> usize { self.pos.get() }

  pub fn get(&self, pos: usize) -> Result<u8, Error> {
    if pos >= self.buf.len() {
      return Err(Error::new(ErrorKind::InvalidInput, "End of buffer get"));
    }
    Ok(self.buf[pos])
  }

  pub fn get_range(&self, pos: usize, len: usize) -> Result<&[u8], Error> {
    if pos + len >= self.buf.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "End of buffer get_range"));
    }
    Ok(&self.buf[pos..pos+len as usize])
  }

  pub fn read_u8(&self) -> Result<u8, Error> {
    if self.pos.get() >= self.buf.len() {
      return Err(Error::new(ErrorKind::InvalidInput, "End of buffer read"));
    }
    let res = self.buf[self.pos.get()];
    self.pos.set(self.pos.get() + 1);
    Ok(res)
  }

  pub fn read_u16(&self) -> Result<u16, Error> {
    let res = ((self.read_u8()? as u16) << 8) |
               (self.read_u8()? as u16);
    Ok(res)
  }

  pub fn read_u32(&self) -> Result<u32, Error> {
    let res = ((self.read_u16()? as u32) << 16) |
              (self.read_u16()? as u32);
    Ok(res)
  }

  pub fn read_name(&self) -> Result<String,Error> {
    let mut name = String::new();
    let mut pos = self.pos();
    let mut delim = "";
    let mut jump = false;

    loop{
      let len = self.get(pos)?;
      pos += 1;

      if len == 0 {
        break;
      }
      else if (len & 0xC0) == 0xC0 {
          // Check if a jump is needed
          if !jump {
            self.pos.set(self.pos.get()+1);
          }

          // Perform jump
          let byte2 = self.get(pos)? as u16;
          let offset = (((len as u16) ^ 0xC0) << 8) | byte2;
          pos = offset as usize;
          jump = true;
      }
      else {
        // Build domain name
        let word = self.get_range(pos, len as usize)?;

        name.push_str(delim);
        name.push_str(&String::from_utf8_lossy(word).to_lowercase());
        delim = ".";

        pos += len as usize;
      }
    }

    if !jump {
      self.pos.set(pos);
    }

    Ok(name)
  }
}
