mod protocol;
use std::io::Error;
use std::net::UdpSocket;
use protocol::Limits;
use protocol::Packet;
use protocol::QueryType;
use protocol::Question;
use protocol::ResultCode;

fn lookup(qname: &str, qtype: QueryType, server: (&str, u16)) -> Result<Packet, Error> {
    let mut request: [u8; Limits::Size as usize] = [0; Limits::Size as usize];
    let mut response: [u8; Limits::Size as usize] = [0; Limits::Size as usize];
    let mut packet = Packet::new();
    let socket = UdpSocket::bind(("0.0.0.0", 43210))?;

    packet.header.id = 6666;
    packet.header.questions = 1;
    packet.header.recursion_desired = true;
    packet.questions.push(Question::build(&qname.to_string(), qtype));
    packet.read(&mut request)?;

    socket.send_to(&mut request, server).unwrap();
    packet.write(&mut response)?;
    Ok(packet)
}

fn main() {
    let server = ("8.8.8.8", 53);
    let local = ("0.0.0.0", 2053);
    let mut array: [u8; Limits::Size as usize] = [0; Limits::Size as usize];
    let mut request = Packet::new();

    let socket = UdpSocket::bind(local).unwrap();

    // Sequentials queries handling
    loop {
        let (_, src) = match socket.recv_from(&mut array) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to read from UDP socket: {:?}", e);
                continue;
            }
        };
        request.read(&mut array).expect("Failed to parse UDP query packet");

        let mut response = Packet::new();
        response.header.id = request.header.id;
        response.header.recursion_desired = true;
        response.header.recursion_available = true;
        response.header.response = true;

        if request.questions.is_empty() {
            response.header.rescode = ResultCode::FORMERR;
        }
        else {
            let question = &request.questions[0];
            println!("Received query: {:?}", question);
        }
    }
}
