use std::io::Read;
use std::fs::File;
use std::net::UdpSocket;

use murdock::protocol::Limits;
use murdock::protocol::Packet;
use murdock::protocol::QueryType;
use murdock::protocol::Question;

#[test]
fn parsing_from_text() {
    // TEST 1
    println!("\n------------------------ TEST PACKET PARSING");
    let mut f = File::open("response_packet.txt").unwrap();
    let mut array: [u8; Limits::Size as usize] = [0; Limits::Size as usize];
    f.read(&mut array).unwrap();

    let mut packet = Packet::new();
    packet.write(&array).unwrap();
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
}

#[test]
fn parsing_from_query() {
    // TEST 2
    // ----------------- SEND ------------------------------
    println!("\n------------------------ TEST SEND/RCV");
    let server = ("8.8.8.8", 53); let socket = UdpSocket::bind(("0.0.0.0", 4321)).unwrap(); // Prepare packet
    let domain_name = "cloudflare.com";
    let qtype = QueryType::A;

    let mut packet = Packet::new();
    packet.header.id = 6666;
    packet.header.recursion_desired = true;
    packet.questions.push(Question::build(domain_name, qtype));
    packet.header.questions = 1;

    let mut send_array: [u8; Limits::Size as usize] = [0; Limits::Size as usize];
    packet.read(&mut send_array).unwrap();

    // send packet
    socket.send_to(&send_array, server).unwrap();


    // ----------------- RECEIVE ---------------------------
    let mut recv_buffer: [u8; Limits::Size as usize] = [0; Limits::Size as usize];
    socket.recv_from(&mut recv_buffer).unwrap();

    let mut recv_packet = Packet::new();
    for elem in recv_buffer.iter() {
        print!("{:#?} ", elem);
    }
    println!("\n LENGTH: {length}", length=recv_buffer.len());
    recv_packet.write(&recv_buffer).unwrap();
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
