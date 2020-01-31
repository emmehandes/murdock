use std::io::Error;
use crate::protocol::querytype::QueryType;
use crate::protocol::buffer::Buffer;

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Question {
  name: String,
  qtype: QueryType
}

impl Question {
  pub fn new(name: String, qtype: QueryType) -> Question {
    Question {
      name: name,
      qtype: qtype
    }
  }

  pub fn read(&mut self, buffer: &mut Buffer) -> Result<(), Error> {
    buffer.get_domain_name(&mut self.name)?;
    self.qtype = QueryType::from_num(buffer.read_u16()?);
    let _ = buffer.read_u16()?;
    Ok(())
  }

  pub fn write(&self, buffer: &mut Buffer) -> Result<(), Error> {
    buffer.set_domain_name(&self.name)?;
    buffer.write_u16(self.qtype.to_num())?;
    buffer.write_u16(1)?;
    Ok(())
  }
}