use std::io::Error;
use crate::protocol::Reader;
use crate::protocol::Writer;
use crate::protocol::Header;
use crate::protocol::Question;
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
    let reader = Reader::new(array);
    self.header.write(&reader)?;

    for _ in 0..self.header.questions {
        let mut question = Question::new();
        question.write(&reader)?;
        self.questions.push(question);
    }

    for _ in 0..self.header.answers {
        self.answers.push(Record::build(&reader)?);
    }
    for _ in 0..self.header.authoritative_entries {
        self.authorities.push(Record::build(&reader)?);
    }
    for _ in 0..self.header.resource_entries {
        self.resources.push(Record::build(&reader)?);
    }

    Ok(())
  }

  pub fn read(&self, array: &mut [u8]) -> Result<(), Error> {
    let mut writer = Writer::new(array);
    self.header.read(&mut writer)?;

    for question in &self.questions {
        question.read(&mut writer)?;
    }
    for answer in &self.answers {
        answer.read(&mut writer)?;
    }
    for authority in &self.authorities {
        authority.read(&mut writer)?;
    }
    for resource in &self.resources {
        resource.read(&mut writer)?;
    }
    Ok(())
  }
}
