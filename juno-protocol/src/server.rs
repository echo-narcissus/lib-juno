use crate::datatypes::*;

// Note: This is plaintext in the sense of being the raw data sent from the client
pub fn parse(plaintext_buffer: &mut Vec<u8>, msg_id_size: u8) -> Result<Operation, String> {
    crate::server_protocol::parse_operation_from_buffer(plaintext_buffer, msg_id_size)
}
