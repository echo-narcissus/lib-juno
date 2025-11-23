mod args;

use juno_protocol::*;
use juno_tls::server::*;
use rand::Rng;

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn main() {
    let args = args::Cli::parse_args();
    let server_config = ServerConfiguration::new(&args.cert, &args.key, &args.bind_addr, args.port).unwrap();

    let storage: RefCell<(HashMap<Vec<u8>, Vec<u8>>, HashMap<Vec<u8>, Vec<u8>>)> = RefCell::new(
        (HashMap::new(), HashMap::new()));

    let stream_callback_cloj = move |buffer: &mut Vec<u8>| -> Option<Operation> { 
        match parse(buffer, args.msg_id_size) {
            Err(e) => {
                eprintln!("{}", e);
                None
            },
            Ok(op) => {
                Some(op)
            }
        }
    };

    let stream_callback_box: Box<dyn FnMut(&mut Vec<u8>) -> Option<Operation>> = 
        Box::new(stream_callback_cloj);
    let stream_callback = Arc::new(Mutex::new(stream_callback_box));


    let op_callback_cloj = move |op: Operation| -> Result<Option<Vec<u8>>, String> {
        execute_operation(op, &mut storage.borrow_mut())
    };

    let op_callback_box: Box<dyn FnMut(Operation) -> Result<Option<Vec<u8>>, String>> = Box::new(op_callback_cloj);
    let operation_callback = Arc::new(Mutex::new(op_callback_box));

    let mut tls_server = TlsServer::new(server_config, args.msg_id_size, stream_callback, operation_callback).expect("couldn't establish TLS server.");
    match tls_server.run(false) {
        Ok(_) => {},
        Err(s) => eprintln!("{}", s)
    }

}

fn execute_operation(op: Operation, storage: &mut (HashMap<Vec<u8>, Vec<u8>>, HashMap<Vec<u8>, Vec<u8>>)) -> Result<Option<Vec<u8>>, String> {
    match op {
        Operation::Retrieve { id } => {
            match storage.0.get(&id) {
                Some(data) => {return Ok(Some(data.clone()))},
                None => {
                    match storage.1.get(&id) {
                        Some(data) => {return Ok(Some(data.clone()))},
                        None => {
                            return Ok(Some(generate_garbage_data()))
                        }
                    }
                }
            }
        }
        Operation::Store { id, data, delete_early } => {
            if delete_early {
                match storage.0.get(&id) {
                    Some(_) => {}
                    None => {
                        storage.0.insert(id, data);
                    }
                }
                Ok(None)
            }
            else {
                match storage.1.get(&id) {
                    Some(_) => {}
                    None => {
                        storage.1.insert(id, data);
                    }
                }
                Ok(None)

            }
        }
    }

}

const MIN_GARBAGE_SIZE: usize = 1024;
const MAX_GARBAGE_SIZE: usize = 16384;
fn generate_garbage_data() -> Vec<u8> {
    let mut rng = rand::rng();
    let len = rng.random_range(MIN_GARBAGE_SIZE..=MAX_GARBAGE_SIZE);
    (0..len).map(|_| rng.random()) .collect()
}
