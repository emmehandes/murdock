use crate::resultcode::ResultCode;

#[derive(Clone,Debug)]
pub struct Header {
  pub id: u16,
  pub recursion_desired: bool,
  pub truncated_message: bool,
  pub authoritative_answer: bool,
  pub opcode: u8,
  pub response: bool,

  pub rescode: ResultCode,
  pub checking_disabled: bool,
  pub authed_data: bool,
  pub z: bool,
  pub recursion_available: bool,

  pub questions: u16,
  pub answers: u16,
  pub authoritative_entries: u16,
  pub resource_entries: u16
}

impl Header {
  pub fn new() -> Header {
    Header { id: 0,
                recursion_desired: false,
                truncated_message: false,
                authoritative_answer: false,
                opcode: 0,
                response: false,

                rescode: ResultCode::NOERROR,
                checking_disabled: false,
                authed_data: false,
                z: false,
                recursion_available: false,

                questions: 0,
                answers: 0,
                authoritative_entries: 0,
                resource_entries: 0 }
  }
}