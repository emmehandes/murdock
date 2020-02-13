use std::io::Error;
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::protocol::Reader;
use crate::protocol::Writer;
use crate::protocol::QueryType;

#[derive(Debug,Clone,PartialEq,Eq,Hash,PartialOrd,Ord)]
#[allow(dead_code)]
pub enum Record {
  UNKNOWN {
    name: String,
    qtype_num: u16,
    data_len: u16,
    ttl: u32
  },
  A {
    name: String,
    addr: Ipv4Addr,
    ttl: u32
  },
  NS {
    name: String,
    host: String,
    ttl: u32
  },
  CNAME {
    name: String,
    host: String,
    ttl: u32
  },
  MX {
    name: String,
    priority: u16,
    host: String,
    ttl: u32
  },
  AAAA {
    name: String,
    addr: Ipv6Addr,
    ttl: u32
  },
}

impl Record {
  pub fn build(reader: &Reader) -> Result<Record, Error> {
    let name = reader.read_name()?;

    let qtype_num = reader.read_u16()?;
    let qtype = QueryType::from_num(qtype_num);
    let _ = reader.read_u16()?; //class is ignored
    let ttl = reader.read_u32()?;
    let data_len = reader.read_u16()?;

    match qtype {
      QueryType::A => {
        let raw_addr = reader.read_u32()?;
        let addr = Ipv4Addr::new(((raw_addr >> 24) & 0xFF) as u8,
                                 ((raw_addr >> 16) & 0xFF) as u8,
                                 ((raw_addr >>  8) & 0xFF) as u8,
                                 ((raw_addr >>  0) & 0xFF) as u8);
        Ok(Record::A { name, addr, ttl })
      },
      QueryType::AAAA => {
        let raw_addr1 = reader.read_u32()?;
        let raw_addr2 = reader.read_u32()?;
        let raw_addr3 = reader.read_u32()?;
        let raw_addr4 = reader.read_u32()?;
        let addr = Ipv6Addr::new((raw_addr1 >> 16) as u16,
                                 (raw_addr1 & 0xFFFF) as u16,
                                 (raw_addr2 >> 16) as u16,
                                 (raw_addr2 & 0xFFFF) as u16,
                                 (raw_addr3 >> 16) as u16,
                                 (raw_addr3 & 0xFFFF) as u16,
                                 (raw_addr4 >> 16) as u16,
                                 (raw_addr4 & 0xFFFF) as u16);
        Ok(Record::AAAA { name, addr, ttl })
      },
      QueryType::NS => {
        let host = reader.read_name()?;
        Ok(Record::NS { name, host, ttl })
      },
      QueryType::CNAME => {
        let host = reader.read_name()?;
        Ok(Record::CNAME { name, host, ttl })
      },
      QueryType::MX => {
        let priority = reader.read_u16()?;
        let host = reader.read_name()?;
        Ok(Record::MX { name, priority, host, ttl })
      },
      QueryType::UNKNOWN(_) => {
        Ok(Record::UNKNOWN { name, qtype_num, data_len, ttl })
      }
    }
  }

  pub fn read(&self, writer: &mut Writer) -> Result<usize, Error> {
    let start_pos = writer.pos();

    match *self {
      Record::A { ref name, ref addr, ttl } => {
        writer.write_name(name)?;
        writer.write_u16(QueryType::A.to_num())?;
        writer.write_u16(1)?;
        writer.write_u32(ttl)?;
        writer.write_u16(4)?;

        let octets = addr.octets();
        writer.write_u8(octets[0])?;
        writer.write_u8(octets[1])?;
        writer.write_u8(octets[2])?;
        writer.write_u8(octets[3])?;
      },
      Record::UNKNOWN { .. } => { println!("Skipping record: {:?}", self); },
      _ => {  }
    }
    Ok(writer.pos() - start_pos)
  }
}
