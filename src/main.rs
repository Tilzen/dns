mod protocol;

use crate::protocol::byte_packet_buffer::BytePacketBuffer;
use crate::protocol::dns_packet::DnsPacket;
use crate::protocol::dns_question::DnsQuestion;
use crate::protocol::query_type::QueryType;
use crate::protocol::types::Result;
use std::net::UdpSocket;

fn main() -> Result<()> {
    let query_name = "google.com";
    let query_type = QueryType::A;

    let server = ("8.8.8.8", 53);

    let socket = UdpSocket::bind(("0.0.0.0", 43210))?;

    let mut packet = DnsPacket::new();

    packet.header.id = 6666;
    packet.header.questions = 1;
    packet.header.recursion_desired = true;
    packet
        .questions
        .push(DnsQuestion::new(query_name.to_string(), query_type));

    let mut request_buffer = BytePacketBuffer::new();
    packet.write(&mut request_buffer)?;

    socket.send_to(&request_buffer.buffer[0..request_buffer.position], server)?;

    let mut response_buffer = BytePacketBuffer::new();
    socket.recv_from(&mut response_buffer.buffer)?;

    let response_packet = DnsPacket::from_buffer(&mut response_buffer)?;
    println!("{:#?}", response_packet.header);

    for question in response_packet.questions {
        println!("{:#?}", question);
    }

    for rec in response_packet.answers {
        println!("{:#?}", rec);
    }

    for rec in response_packet.authorities {
        println!("{:#?}", rec);
    }

    for rec in response_packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}
