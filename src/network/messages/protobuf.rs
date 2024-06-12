/*
Libp2p uses protocol buffers extensively for message serialization
*/

use prost::Message;
use std::io::{self, Read, Write};

// To decode a message
pub fn decode_protobuf<R, M>(mut reader: R) -> io::Result<M>
where
    R: Read,
    M: Message + Default, 
{
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf)?;
    let len = u32::from_le_bytes(len_buf) as usize;

    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf)?;
    M::decode(&*buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

// To encode a message
pub fn encode_protobuf<W, M>(mut writer: W, msg: &M) -> io::Result<()>
where
    W: Write,
    M: Message, 
{
    let mut buf = Vec::new();
    msg.encode(&mut buf)?;

    let len = (buf.len() as u32).to_le_bytes();
    writer.write_all(&len)?;
    writer.write_all(&buf)?;
    Ok(())
}