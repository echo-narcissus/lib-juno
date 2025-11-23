use crate::datatypes::*;
use crate::protocol_constants::*;

pub(crate) fn parse_operation_from_buffer(
    buffer: &mut Vec<u8>,
    msg_id_size: u8,
) -> Result<Operation, String> {
    if buffer.is_empty() {
        return Err("Buffer is empty.".to_string());
    }
    let op_type = buffer[0];
    match op_type {
        OP_STORE => parse_store_operation(buffer, msg_id_size),
        OP_RETRIEVE => parse_retrieve_operation(buffer, msg_id_size),
        _ => {
            // Invalid op_type, clear buffer to prevent looping
            // TODO: This will lead to dropped messages, which is not good
            
            buffer.clear();
            Err(format!("Invalid operation type: {}", op_type))
        }
    }
}

fn parse_store_operation(buffer: &mut Vec<u8>, msg_id_size: u8) -> Result<Operation, String> {
    let err_string = "Buffer too small to parse store operation: ".to_string();
    // Header length = 1 (op) + N (msg id) + 4 (data length)
    let header_len = 1 + (msg_id_size as usize) + 4;
    if buffer.len() < header_len {
        return Err(err_string + "Not enough data for header.");
    }

    let len_bytes: [u8; 4] = buffer[1 + (msg_id_size as usize)..header_len]
        .try_into()
        .unwrap();
    let data_len = u32::from_be_bytes(len_bytes) as usize;

    //add one for the expiry flag, at the end of the message
    let body_len = data_len + 1;
    if buffer.len() < header_len + body_len {
        return Err(err_string + "Not enough data for body.");
    }

    let total_len = header_len + body_len;

    let delete_early = matches!(buffer[(header_len + data_len)], EXPIRE_EARLY);
    
    let op_bytes = buffer.drain(..total_len).collect::<Vec<u8>>();

    let id = op_bytes[1..1 + (msg_id_size as usize)].to_vec();
    let data = op_bytes[header_len..header_len + data_len].to_vec();

    Ok(Operation::Store {
        id,
        data,
        delete_early,
    })
}

fn parse_retrieve_operation(buffer: &mut Vec<u8>, msg_id_size: u8) -> Result<Operation, String> {
    // Message length = 1(op) + n (msg id)
    let total_len = 1 + (msg_id_size as usize);
    if buffer.len() < total_len {
        return Err("Buffer too small to parse retrieve operation.".to_string());
    }
    let mut op_bytes = buffer.drain(..total_len).collect::<Vec<u8>>();
    let id = op_bytes.split_off(1);
    Ok(Operation::Retrieve { id })
}
