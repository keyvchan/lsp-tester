use std::{
    io,
    net::{TcpStream, ToSocketAddrs},
};

use crossbeam_channel::{Receiver, Sender};
use log::debug;
use lsp_types::{ClientCapabilities, InitializeParams, InitializedParams};

use crate::{
    error::ProtocolError,
    msg::{self, Message},
    socket,
    stdio::IoThreads,
};

/// Connection is just a pair of channels of LSP messages.
pub struct Connection {
    pub sender: Sender<Message>,
    pub receiver: Receiver<Message>,
}

impl Connection {
    /// Open a connection over tcp.
    /// This call blocks until a connection is established.
    ///
    /// Use this to connect to a real language server.
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<(Connection, IoThreads)> {
        let stream = TcpStream::connect(addr)?;
        let (sender, receiver, io_threads) = socket::socket_transport(stream);
        Ok((Connection { sender, receiver }, io_threads))
    }

    pub fn initialize_start(&mut self) {
        debug!("initialize_start");
        let params = InitializeParams {
            process_id: None,
            root_path: None,
            root_uri: None,
            initialization_options: None,
            capabilities: ClientCapabilities::default(),
            trace: None,
            workspace_folders: None,
            client_info: None,
            locale: None,
        };

        let request = msg::Request::new(msg::RequestId::from(1), "initialize".to_string(), params);

        self.sender.send(Message::Request(request)).unwrap();

        debug!("sent initialize request");
    }

    /// Finishes the initialization process by sending an `InitializeResult` to the client
    pub fn initialize_finish(&self) -> Result<(), ProtocolError> {
        debug!("receive initialize result");
        let resp = self.receiver.recv().unwrap();
        match resp {
            Message::Response(_) => {
                // consider that the initialization is finished
                debug!("received response");
            }
            Message::Notification(notification) => {
                // the initialization is not finished
                debug!("received notification: {:?}", notification);
                return Err(ProtocolError(format!(
                    "Unexpected notification: {:?}",
                    notification
                )));
            }
            _ => {
                // unknown error occurred
                debug!("received unexpected message");
                return Err(ProtocolError(format!("Unexpected message: {:?}", resp)));
            }
        }
        // send initialized notification
        let notification = msg::Notification::new("initialized".to_string(), InitializedParams {});

        self.sender
            .send(Message::Notification(notification))
            .unwrap();
        debug!("sent initialized notification");
        Ok(())
    }
}
