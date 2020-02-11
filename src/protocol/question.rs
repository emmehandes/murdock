use std::io::Error;
use crate::protocol::QueryType;
use crate::protocol::Buffer;

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Question {
  name: String,
  qtype: QueryType
}

impl Question {
  pub fn from_buffer(buffer: &mut Buffer) -> Result<Question, Error> {
    Question{ name: buffer.read_name()?,
              qtype: QueryType::from_num(buffer.read_u16()?)}
  }

  pub fn to_buffer(&self, buffer: &mut Buffer) -> Result<(), Error> {
    buffer.write_name(&self.name)?;
    buffer.write_u16(self.qtype.to_num())?;
    buffer.write_u16(1)?;
    Ok(())
  }
}
