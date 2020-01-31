
mod protocol;
use std::io::Read;
use std::fs::File;
use crate::protocol::buffer::Buffer;
use crate::protocol::packet::Packet;

fn main() {
    let mut f = File::open("response_packet.txt").unwrap();
    let mut buffer = Buffer::new();
    f.read(&mut buffer.buf).unwrap();

    let packet = Packet::from_buffer(&mut buffer).unwrap();
    println!("{:?}", packet.header);

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
