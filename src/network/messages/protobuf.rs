/*
Libp2p uses protocol buffers extensively for message serialization
*/

use crate::network::messages::message::Message;
use std::io::{self, Read, Write};
use bytes::{Buf, BufMut};

// `Message` is a prost-generated struct from message.proto files

// decodes msg to r
pub fn decode_proto<R: Read>(mut reader: R) -> io::Result<Message> {
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf)?;
    let len = u32::from_le_bytes(len_buf) as usize;

    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf)?;
    Message::decode(&*buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

// encodes msg to w
pub fn encode_proto<W: Write>(mut writer: W, msg: &Message) -> io::Result<()> {
    let mut buf = Vec::new();
    msg.encode(&mut buf)?;

    let len = (buf.len() as u32).to_le_bytes();
    writer.write_all(&len)?;
    writer.write_all(&buf)?;
    Ok(())
}