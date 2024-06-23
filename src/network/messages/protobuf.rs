use crate::network::messages::message::{Message, Transaction, Block, Header};
use crate::network::messages::message::message::Payload;
use std::io::{self, Error, ErrorKind};

// Encode a Message into a Vec<u8>
pub fn encode_message(msg: &Message) -> Vec<u8> {
    let mut result = Vec::new();
    
    match &msg.payload {
        Some(Payload::Transaction(transaction)) => {
            // Field number 1, wire type 2 (length-delimited)
            result.extend_from_slice(&[10]);
            let encoded_transaction = encode_transaction(transaction);
            encode_varint(encoded_transaction.len() as u64, &mut result);
            result.extend_from_slice(&encoded_transaction);
        }
        Some(Payload::Block(block)) => {
            // Field number 2, wire type 2 (length-delimited)
            result.extend_from_slice(&[18]);
            let encoded_block = encode_block(block);
            encode_varint(encoded_block.len() as u64, &mut result);
            result.extend_from_slice(&encoded_block);
        }
        None => {}
    }
    
    result
}

// Decode a Vec<u8> into a Message
pub fn decode_message(bytes: &[u8]) -> io::Result<Message> {
    let mut index = 0;
    let mut msg = Message { payload: None };
    
    while index < bytes.len() {
        let (field_number, wire_type) = decode_key(&mut index, bytes)?;
        match (field_number, wire_type) {
            (1, 2) => { // Transaction
                let len = decode_varint(&mut index, bytes)? as usize;
                let transaction = decode_transaction(&bytes[index..index+len])?;
                msg.payload = Some(Payload::Transaction(transaction));
                index += len;
            }
            (2, 2) => { // Block
                let len = decode_varint(&mut index, bytes)? as usize;
                let block = decode_block(&bytes[index..index+len])?;
                msg.payload = Some(Payload::Block(block));
                index += len;
            }
            _ => return Err(Error::new(ErrorKind::InvalidData, "Unknown field")),
        }
    }
    
    Ok(msg)
}

fn encode_transaction(transaction: &Transaction) -> Vec<u8> {
    let mut result = Vec::new();
    // Field number 1, wire type 0 (varint)
    result.extend_from_slice(&[8]);
    encode_varint(transaction.nonce, &mut result);
    result
}

fn decode_transaction(bytes: &[u8]) -> io::Result<Transaction> {
    let mut index = 0;
    let mut transaction = Transaction { nonce: 0 };
    
    while index < bytes.len() {
        let (field_number, wire_type) = decode_key(&mut index, bytes)?;
        match (field_number, wire_type) {
            (1, 0) => { // nonce
                transaction.nonce = decode_varint(&mut index, bytes)?;
            }
            _ => return Err(Error::new(ErrorKind::InvalidData, "Unknown field in Transaction")),
        }
    }
    
    Ok(transaction)
}

fn encode_block(block: &Block) -> Vec<u8> {
    let mut result = Vec::new();
    
    if let Some(header) = &block.header {
        // Field number 1, wire type 2 (length-delimited)
        result.extend_from_slice(&[10]);
        let encoded_header = encode_header(header);
        encode_varint(encoded_header.len() as u64, &mut result);
        result.extend_from_slice(&encoded_header);
    }
    
    for transaction in &block.transactions {
        // Field number 2, wire type 2 (length-delimited)
        result.extend_from_slice(&[18]);
        let encoded_transaction = encode_transaction(transaction);
        encode_varint(encoded_transaction.len() as u64, &mut result);
        result.extend_from_slice(&encoded_transaction);
    }
    
    result
}

fn decode_block(bytes: &[u8]) -> io::Result<Block> {
    let mut index = 0;
    let mut block = Block {
        header: None,
        transactions: Vec::new(),
    };
    
    while index < bytes.len() {
        let (field_number, wire_type) = decode_key(&mut index, bytes)?;
        match (field_number, wire_type) {
            (1, 2) => { // header
                let len = decode_varint(&mut index, bytes)? as usize;
                block.header = Some(decode_header(&bytes[index..index+len])?);
                index += len;
            }
            (2, 2) => { // transaction
                let len = decode_varint(&mut index, bytes)? as usize;
                let transaction = decode_transaction(&bytes[index..index+len])?;
                block.transactions.push(transaction);
                index += len;
            }
            _ => return Err(Error::new(ErrorKind::InvalidData, "Unknown field in Block")),
        }
    }
    
    Ok(block)
}

fn encode_header(header: &Header) -> Vec<u8> {
    let mut result = Vec::new();
    
    // Field number 1, wire type 0 (varint)
    result.extend_from_slice(&[8]);
    encode_varint(header.index as u64, &mut result);
    
    // Field number 2, wire type 0 (varint)
    result.extend_from_slice(&[16]);
    encode_varint(header.nonce, &mut result);
    
    result
}

fn decode_header(bytes: &[u8]) -> io::Result<Header> {
    let mut index = 0;
    let mut header = Header { index: 0, nonce: 0 };
    
    while index < bytes.len() {
        let (field_number, wire_type) = decode_key(&mut index, bytes)?;
        match (field_number, wire_type) {
            (1, 0) => { // index
                header.index = decode_varint(&mut index, bytes)? as u32;
            }
            (2, 0) => { // nonce
                header.nonce = decode_varint(&mut index, bytes)?;
            }
            _ => return Err(Error::new(ErrorKind::InvalidData, "Unknown field in Header")),
        }
    }
    
    Ok(header)
}

fn encode_varint(value: u64, output: &mut Vec<u8>) {
    let mut value = value;
    while value >= 0b1000_0000 {
        output.push((value & 0b0111_1111 | 0b1000_0000) as u8);
        value >>= 7;
    }
    output.push(value as u8);
}

fn decode_varint(index: &mut usize, bytes: &[u8]) -> io::Result<u64> {
    let mut result = 0u64;
    let mut shift = 0;
    
    loop {
        if *index >= bytes.len() {
            return Err(Error::new(ErrorKind::UnexpectedEof, "Unexpected end of input"));
        }
        
        let byte = bytes[*index];
        *index += 1;
        
        result |= ((byte & 0b0111_1111) as u64) << shift;
        shift += 7;
        
        if byte & 0b1000_0000 == 0 {
            break;
        }
    }
    
    Ok(result)
}

fn decode_key(index: &mut usize, bytes: &[u8]) -> io::Result<(u32, u8)> {
    let varint = decode_varint(index, bytes)?;
    let wire_type = (varint & 0b111) as u8;
    let field_number = (varint >> 3) as u32;
    Ok((field_number, wire_type))
}

// Public encode and decode functions
pub fn encode_protobuf(message: &Message) -> io::Result<Vec<u8>> {
    Ok(encode_message(message))
}

pub fn decode_protobuf(bytes: &[u8]) -> io::Result<Message> {
    decode_message(bytes)
}