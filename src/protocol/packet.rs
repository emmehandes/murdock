use std::io::Error;
use crate::protocol::PacketBuffer;
use crate::protocol::Header;
use crate::protocol::Question;
use crate::protocol::QueryType;
use crate::protocol::Record;

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

  pub fn write(&mut self, array: &[u8]) -> Result<(), Error> {
    let mut packet_buffer = PacketBuffer::build(array);
    self.header.write(packet_buffer)?;

    for _ in 0..self.header.questions {
        let mut question = Question::new();
        question.write(packet_buffer)?;
        self.questions.push(question);
    }

    for _ in 0..self.header.answers {
        self.answers.push(Record::build(packet_buffer)?);
    }
    for _ in 0..self.header.authoritative_entries {
        self.authorities.push(Record::build(packet_buffer)?);
    }
    for _ in 0..self.header.resource_entries {
        self.resources.push(Record::build(packet_buffer)?);
    }

    Ok(())
  }

  pub fn read(&self, buffer: &mut PacketBuffer) -> Result<(), Error> {
    self.header.read(buffer)?;

    for question in &self.questions {
        question.read(buffer)?;
    }
    for answer in &self.answers {
        answer.read(buffer)?;
    }
    for authority in &self.authorities {
        authority.read(buffer)?;
    }
    for resource in &self.resources {
        resource.read(buffer)?;
    }
    Ok(())
  }
}
