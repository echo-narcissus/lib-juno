use crate::datatypes::*;

pub fn generate(op: Operation) -> Result<Vec<u8>, String> {
    crate::client_protocol::create_buffer_from_operation(&op)
}
