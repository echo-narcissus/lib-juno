use crate::datatypes::*;
use crate::protocol_constants::*;

pub(crate) fn create_buffer_from_operation(op: &Operation) -> Result<Vec<u8>, String> {
    match op {
        Operation::Store {
            id,
            data,
            delete_early,
        } => create_buffer_from_store_op(id, data, delete_early),
        Operation::Retrieve { id } => create_buffer_from_retrieve_op(id),
    }
}

fn create_buffer_from_store_op(id: &[u8], data: &[u8], delete_early: &bool) -> Result<Vec<u8>, String> {
    if data.len() > u32::MAX as usize {
        return Err("trying to create a buffer with too much data(> 4Gb)".to_string());
    }

    // 6 = 1 (opcode) + 4 (length) + 1 (delete early)
    let mut buffer: Vec<u8> = Vec::with_capacity(6 + id.len() + data.len());
    buffer[0] = OP_STORE;
    buffer[1..(1 + id.len())].copy_from_slice(id);
    buffer[(1 + id.len())..(5 + id.len())].copy_from_slice(&(data.len() as u32).to_be_bytes());
    buffer[(5 + id.len())..(5 + data.len() + id.len())].copy_from_slice(data);
    buffer[5 + data.len() + id.len()] = if *delete_early { EXPIRE_EARLY } else { 0x00 };
    Ok(buffer)
}

fn create_buffer_from_retrieve_op(id: &[u8]) -> Result<Vec<u8>, String> {
    let mut buffer: Vec<u8> = Vec::with_capacity(1 + id.len());
    buffer[0] = OP_STORE;
    buffer[1..(1 + id.len())].copy_from_slice(id);
    Ok(buffer)
}
