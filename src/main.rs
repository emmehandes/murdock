pub mod buffer;
pub mod header;
pub mod resultcode;
pub mod querytype;
pub mod question;
pub mod record;
pub mod packet;

use std::fs::File;
use crate::buffer::Buffer;
use crate::packet::Packet;

fn main() {
    let mut f = File::open("response_packet.txt").unwrap();
    let mut buffer = Buffer::new();

    let packet = Packet::from_buffer(&mut buffer).unwrap();

    for q in packet.questions {
        println!("{:?}", q);
    }
    for rec in packet.answers {
        println!("{:?}", rec);
    }
    for rec in packet.authorities {
        println!("{:?}", rec);
    }
    for rec in packet.resources {
        println!("{:?}", rec);
    }
}
