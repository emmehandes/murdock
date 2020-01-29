use std::io::{Error, ErrorKind};

pub struct Buffer {
  pub buf: [u8; 512],
  pos: usize
}

impl Buffer {

  pub fn new() -> Buffer {
    Buffer { buf: [0; 512], pos: 0 }
  }

  fn get(&mut self, pos: usize) -> Result<u8, Error> {
    if pos >= 512 {
      return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
    }
    Ok(self.buf[pos])
  }

  fn get_range(&mut self, pos: usize, len: usize) -> Result<&[u8], Error> {
    if pos + len >= 512 {
        return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
    }
    Ok(&self.buf[pos..pos+len as usize])
  }

  fn read_u8(&mut self) -> Result<u8, Error> {
    if self.pos >= 512 {
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
    let res = ((self.read_u8()? as u32) << 24) |
              ((self.read_u8()? as u32) << 16) |
              ((self.read_u8()? as u32) << 8) |
              ((self.read_u8()? as u32) << 0);
    Ok(res)
  }

  pub fn get_domain_name(&mut self, domain_name: &mut String) -> Result<(),Error> {
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
            self.pos = pos+1;
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

        domain_name.push_str(delim);
        domain_name.push_str(&String::from_utf8_lossy(word).to_lowercase());
        delim = ".";

        pos += len as usize;
      }
    }

    if !jump {
      self.pos = pos;
    }
    Ok(())
  }
}