use crate::querytype::QueryType;
use crate::buffer::Buffer;
use std::io::Error;

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
}