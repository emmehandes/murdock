use std::io::Error;
use crate::protocol::buffer::Buffer;
use crate::protocol::header::Header;
use crate::protocol::question::Question;
use crate::protocol::querytype::QueryType;
use crate::protocol::record::Record;

#[derive(Clone, Debug)]
pub struct Packet {
  pub header: Header,
  pub questions: Vec<Question>,
  pub answers: Vec<Record>,
  pub authorities: Vec<Record>,
  pub resources: Vec<Record>
}

impl Packet {
  pub fn new() -> Packet {
    Packet {
      header: Header::new(),
      questions: Vec::new(),
      answers: Vec::new(),
      authorities: Vec::new(),
      resources: Vec::new(),
    }
  }

  // TODO: remove mutability of the buffer, a reading should not alter the buffer
  pub fn read(buffer: &mut Buffer) -> Result<Packet, Error> {
    let mut result = Packet::new();
    result.header.read(buffer)?;

    for _ in 0..result.header.questions {
        let mut question = Question::new("".to_string(),
                                         QueryType::UNKNOWN(0));
        question.read(buffer)?;
        result.questions.push(question);
    }

    for _ in 0..result.header.answers {
        let rec = Record::read(buffer)?;
        result.answers.push(rec);
    }
    for _ in 0..result.header.authoritative_entries {
        let rec = Record::read(buffer)?;
        result.authorities.push(rec);
    }
    for _ in 0..result.header.resource_entries {
        let rec = Record::read(buffer)?;
        result.resources.push(rec);
    }

    Ok(result)
  }

  pub fn write(&self, buffer: &mut Buffer) -> Result<(), Error> {
    self.header.write(buffer)?;

    for question in &self.questions {
        question.write(buffer)?;
    }
    for answer in &self.answers {
        answer.write(buffer)?;
    }
    for authority in &self.authorities {
        authority.write(buffer)?;
    }
    for resource in &self.resources {
        resource.write(buffer)?;
    }
    Ok(())
  }
}
