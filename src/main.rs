mod protocol;
use std::io::Read;
use std::fs::File;
use std::net::UdpSocket;
use protocol::Buffer;
use protocol::Packet;
use protocol::QueryType;
use protocol::Question;

fn main() {
    // TEST 1
    println!("\n------------------------ TEST PACKET PARSING");
    let mut f = File::open("response_packet.txt").unwrap();
    let mut buffer = Buffer::new();
    f.read(&mut buffer.buf).unwrap();

    let packet = Packet::read(&mut buffer).unwrap();
    println!("{:#?}", packet.header);

    for q in packet.questions {
        println!("{:#?}", q);
    }
    for rec in packet.answers {
        println!("{:#?}", rec);
    }
    for rec in packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in packet.resources {
        println!("{:#?}", rec);
    }

    // TEST 2
    // ----------------- SEND ------------------------------
    println!("\n------------------------ TEST SEND/RCV");
    let server = ("8.8.8.8", 53);
    let socket = UdpSocket::bind(("0.0.0.0", 4321)).unwrap();

    // Prepare packet
    let domain_name = "cloudflare.com";
    let qtype = QueryType::A;

    let mut packet = Packet::new();
    packet.header.id = 6666;
    packet.header.recursion_desired = true;
    packet.questions.push(Question::new(domain_name.to_string(), qtype));
    packet.header.questions = 1;

    let mut send_buffer = Buffer::new();
    packet.write(&mut send_buffer).unwrap();

    // send packet
    socket.send_to(&send_buffer.buf[0..send_buffer.pos()], server).unwrap();


    // ----------------- RECEIVE ---------------------------
    let mut recv_buffer = Buffer::new();
    socket.recv_from(&mut recv_buffer.buf).unwrap();

    let recv_packet = Packet::read(&mut recv_buffer).unwrap();
    println!("{:#?}", recv_packet.header);

    for q in recv_packet.questions {
        println!("{:#?}", q);
    }
    for rec in recv_packet.answers {
        println!("{:#?}", rec);
    }
    for rec in recv_packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in recv_packet.resources {
        println!("{:#?}", rec);
    }
}
