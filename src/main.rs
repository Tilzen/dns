mod protocol;

use crate::protocol::byte_packet_buffer::BytePacketBuffer;
use crate::protocol::dns_packet::DnsPacket;
use crate::protocol::types::Result;
use std::fs::File;
use std::io::Read;

fn main() -> Result<()> {
    let mut file = File::open("response_packet.txt")?;
    let mut packet_buffer = BytePacketBuffer::new();

    file.read(&mut packet_buffer.buffer)?;

    let packet = DnsPacket::from_buffer(&mut packet_buffer)?;
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

    Ok(())
}
