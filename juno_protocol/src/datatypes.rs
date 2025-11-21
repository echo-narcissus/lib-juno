#[derive(Debug)]
pub enum Operation {
    Store {
        id: Vec<u8>,
        data: Vec<u8>,
        delete_early: bool,
    },
    Retrieve {
        id: Vec<u8>,
    },
}

pub enum Response {
    Store,
    Retrieve { data: Vec<u8> },
}
