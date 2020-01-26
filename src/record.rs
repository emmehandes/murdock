use std::io::Error;
use crate::buffer::Buffer;
use crate::querytype::QueryType;
use std::net::Ipv4Addr;

#[derive(Debug,Clone,PartialEq,Eq,Hash,PartialOrd,Ord)]
#[allow(dead_code)]
pub enum Record {
  UNKNOWN {
    domain: String,
    qtype: u16,
    data_len: u16,
    ttl: u32
  },
  A {
    domain: String,
    addr: Ipv4Addr,
    ttl: u32
  },
}

impl Record {
  pub fn read(buffer: &mut Buffer) -> Result<Record, Error> {
      let mut domain = String::new();
      buffer.get_domain_name(&mut domain)?;

      let qtype_num = buffer.read_u16()?;
      let qtype = QueryType::from_num(qtype_num);
      let _ = buffer.read_u16()?; //class is ignored
      let ttl = buffer.read_u32()?;
      let data_len = buffer.read_u16()?;

      match qtype {
        QueryType::A => {
          let raw_addr = buffer.read_u32()?;
          let addr = Ipv4Addr::new(((raw_addr >> 24) & 0xFF) as u8,
                                   ((raw_addr >> 16) & 0xFF) as u8,
                                   ((raw_addr >>  8) & 0xFF) as u8,
                                   ((raw_addr >>  0) & 0xFF) as u8);
          Ok(Record::A {
              domain: domain,
              addr: addr,
              ttl: ttl
          })
        },
        QueryType::UNKNOWN(_) => {
          Ok(Record::UNKNOWN {
            domain: domain,
            qtype: qtype_num,
            data_len: data_len,
            ttl: ttl
          })
        }
      }

  }
}