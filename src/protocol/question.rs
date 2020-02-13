use std::io::Error;
use crate::protocol::QueryType;
use crate::protocol::Reader;
use crate::protocol::Writer;

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Question {
  name: String,
  qtype: QueryType
}

impl Question {
  pub fn new() -> Question {
    Question { name: "".to_string(), qtype: QueryType::UNKNOWN(0) }
  }

  pub fn build(name: &str, qtype: QueryType) -> Question {
    Question { name: name.to_string(), qtype }
  }

  pub fn write(&mut self, reader: &Reader) -> Result<(), Error> {
    self.name = reader.read_name()?;
    self.qtype = QueryType::from_num(reader.read_u16()?);
    Ok(())
  }

  pub fn read(&self, writer: &mut Writer) -> Result<(), Error> {
    writer.write_name(&self.name)?;
    writer.write_u16(self.qtype.to_num())?;
    writer.write_u16(1)?;
    Ok(())
  }
}
