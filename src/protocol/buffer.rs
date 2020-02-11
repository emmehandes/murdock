use std::io::{Error, ErrorKind};

pub struct PacketBuffer<'a> {
  buf: &'a [u8],
  pos: usize,
}

impl<'a> PacketBuffer<'a> {
  pub const MAX_SIZE: usize=512;
  pub const MAX_LABEL_LEN: usize=0x34;

  pub fn build(src: &[u8]) -> PacketBuffer {
    PacketBuffer { buf: src, pos: 0}
  }

  pub fn new() -> PacketBuffer<'a>  {
    PacketBuffer { buf: &[0; PacketBuffer::MAX_SIZE], pos: 0}
  }

  pub fn pos(&self) -> usize { self.pos }

  pub fn get(&self, pos: usize) -> Result<u8, Error> {
    if pos >= self.buf.len() {
      return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
    }
    Ok(self.buf[pos])
  }

  pub fn get_range(&self, pos: usize, len: usize) -> Result<&[u8], Error> {
    if pos + len >= self.buf.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
    }
    Ok(&self.buf[pos..pos+len as usize])
  }

  pub fn read_u8(&mut self) -> Result<u8, Error> {
    if self.pos >= self.buf.len() {
      return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
    }
    let res = self.buf[self.pos];
    self.pos += 1;
    Ok(res)
  }

  pub fn read_u16(&mut self) -> Result<u16, Error> {
    let res = ((self.read_u8()? as u16) << 8) |
               (self.read_u8()? as u16);
    Ok(res)
  }

  pub fn read_u32(&mut self) -> Result<u32, Error> {
    let res = ((self.read_u16()? as u32) << 16) |
              (self.read_u16()? as u32);
    Ok(res)
  }

  pub fn read_name(&mut self) -> Result<String,Error> {
    let mut name = String::new();
    let mut pos = self.pos;
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
            self.pos += 1;
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
      self.pos = pos;
    }

    Ok(name)
  }


  pub fn write_u8(&mut self, val: u8) -> Result<(), Error> {
    if self.pos >= self.buf.len() {
      return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
    }
    self.buf[self.pos] = val;
    self.pos += 1;
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
      if label_len > PacketBuffer::MAX_LABEL_LEN {
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
