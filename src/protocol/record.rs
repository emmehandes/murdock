use std::io::Error;
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::protocol::buffer::Buffer;
use crate::protocol::querytype::QueryType;

#[derive(Debug,Clone,PartialEq,Eq,Hash,PartialOrd,Ord)]
#[allow(dead_code)]
pub enum Record {
  UNKNOWN {
    domain_name: String,
    qtype_num: u16,
    data_len: u16,
    ttl: u32
  },
  A {
    domain_name: String,
    addr: Ipv4Addr,
    ttl: u32
  },
  NS {
    domain_name: String,
    host: String,
    ttl: u32
  },
  CNAME {
    domain_name: String,
    host: String,
    ttl: u32
  },
  MX {
    domain_name: String,
    priority: u16,
    host: String,
    ttl: u32
  },
  AAAA {
    domain_name: String,
    addr: Ipv6Addr,
    ttl: u32
  },
}

impl Record {
  pub fn read(buffer: &mut Buffer) -> Result<Record, Error> {
    let mut domain_name = String::new();
    buffer.get_domain_name(&mut domain_name)?;

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
        Ok(Record::A { domain_name, addr, ttl })
      },
      QueryType::AAAA => {
        let raw_addr1 = buffer.read_u32()?;
        let raw_addr2 = buffer.read_u32()?;
        let raw_addr3 = buffer.read_u32()?;
        let raw_addr4 = buffer.read_u32()?;
        let addr = Ipv6Addr::new((raw_addr1 >> 16) as u16,
                                 (raw_addr1 & 0xFFFF) as u16,
                                 (raw_addr2 >> 16) as u16,
                                 (raw_addr2 & 0xFFFF) as u16,
                                 (raw_addr3 >> 16) as u16,
                                 (raw_addr3 & 0xFFFF) as u16,
                                 (raw_addr4 >> 16) as u16,
                                 (raw_addr4 & 0xFFFF) as u16);
        Ok(Record::AAAA { domain_name, addr, ttl })
      },
      QueryType::NS => {
        let mut host = String::new();
        buffer.get_domain_name(&mut host)?;
        Ok(Record::NS { domain_name, host, ttl })
      },
      QueryType::CNAME => {
        let mut host = String::new();
        buffer.get_domain_name(&mut host)?;
        Ok(Record::CNAME { domain_name, host, ttl })
      },
      QueryType::MX => {
        let priority = buffer.read_u16()?;
        let mut host = String::new();
        buffer.get_domain_name(&mut host)?;
        Ok(Record::MX { domain_name, priority, host, ttl })
      },
      QueryType::UNKNOWN(_) => {
        Ok(Record::UNKNOWN { domain_name, qtype_num, data_len, ttl })
      }
    }
  }

  pub fn write(&self, buffer: &mut Buffer) -> Result<usize, Error> {
    let start_pos = buffer.pos();

    match *self {
      Record::A { ref domain_name, ref addr, ttl } => {
        buffer.set_domain_name(domain_name)?;
        buffer.write_u16(QueryType::A.to_num())?;
        buffer.write_u16(1)?;
        buffer.write_u32(ttl)?;
        buffer.write_u16(4)?;

        let octets = addr.octets();
        buffer.write_u8(octets[0])?;
        buffer.write_u8(octets[1])?;
        buffer.write_u8(octets[2])?;
        buffer.write_u8(octets[3])?;
      },
      Record::UNKNOWN { .. } => { println!("Skipping record: {:?}", self); },
      _ => {  }
    }
    Ok(buffer.pos() - start_pos)
  }
}
