use crate::server::config::ServerConfiguration;

use crate::server::server_connection::Connection;

use juno_protocol::Operation;
use mio::event::Event;
use mio::net::TcpListener;
use mio::{Events, Interest, Poll, Token};
use rustls::{ServerConfig};
use slab::Slab;
use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;



const SERVER_TOKEN: Token = Token(usize::MAX);

type StreamCallback = Arc<Mutex<Box<dyn FnMut(&mut Vec<u8>) -> Option<Operation>>>>;
type OperationCallback = Arc<Mutex<Box<dyn FnMut(Operation) -> Result<Option<Vec<u8>>, String>>>>;

pub struct TlsServer {
    server_socket: TcpListener,
    poll: Poll,
    connections: Slab<Connection>,
    tls_config: Arc<ServerConfig>,
    msg_id_size: u8,
    stream_callback: StreamCallback,
    operation_callback: OperationCallback,
    
}

impl TlsServer {
    pub fn new(
        config: ServerConfiguration,
        msg_id_size: u8,
        stream_callback: StreamCallback,
        operation_callback: OperationCallback 
    ) -> io::Result<Self> {
        let mut server_socket = TcpListener::bind(config.socket_addr)?;
        let poll = Poll::new()?;

        poll.registry().register(
            &mut server_socket,
            SERVER_TOKEN,
            Interest::READABLE,
        )?;

        Ok(Self {
            server_socket,
            poll,
            connections: Slab::with_capacity(1024),
            tls_config: config.tls_config,
            msg_id_size,
            stream_callback,
            operation_callback
        })
    }

    pub fn run(&mut self, verbose: bool) -> io::Result<()> {
        let mut events = Events::with_capacity(1024);
        loop {
            self.poll.poll(&mut events, Some(Duration::from_secs(1)))?;

            for event in &events {
                match event.token() {
                    SERVER_TOKEN => {
                        // New connections
                        self.accept_connections(verbose)?;
                    }
                    token => {
                        // Event on existing connection
                        self.handle_connection_event(token, event)?;
                    }
                }
            }
        }
    }

    fn accept_connections(&mut self, verbose: bool) -> io::Result<()> {
        loop {
            match self.server_socket.accept() {
                Ok((socket, addr)) => {
                    println!("Accepted new connection from: {}", addr);

                    if self.connections.len() >= self.connections.capacity() {
                        eprintln!("Connection slab is full, dropping connection");
                        // Dropping the socket will close it.
                        continue;
                    }

                    let entry = self.connections.vacant_entry();
                    let token = Token(entry.key());

                    let connection = Connection::new(
                        socket,
                        token,
                        self.tls_config.clone(),
                        self.msg_id_size,
                        verbose
                    )?;

                    entry.insert(connection).register(&mut self.poll);
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // No more connections pending
                    break;
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    // Handle an event for a specific connection token.
    fn handle_connection_event(&mut self, token: Token, event: &Event) -> io::Result<()> {
        if let Some(conn) = self.connections.get_mut(token.0) {
            conn.ready(&mut self.poll, event, 
                &mut *self.stream_callback.lock().unwrap(), 
                &mut *self.operation_callback.lock().unwrap());

            if conn.is_closed() {
                // Connection closed, remove it.
                println!("Connection closed for token: {}", token.0);
                self.connections.remove(token.0);
            }
        } else {
            // token not in slab
        }
        Ok(())
    }
}
