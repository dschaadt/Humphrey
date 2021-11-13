use crate::error::WebsocketError;
use crate::frame::{Frame, Opcode};
use crate::message::Message;

use std::io::{Read, Write};

/// Represents a WebSocket stream.
pub struct WebsocketStream<T>
where
    T: Read + Write,
{
    stream: T,
    closed: bool,
}

impl<T> WebsocketStream<T>
where
    T: Read + Write,
{
    /// Creates a new `WebsocketStream` wrapping an underlying stream, usually `TcpStream`.
    ///
    /// When the `WebsocketStream` is dropped, a close frame will be sent to the client.
    pub fn new(stream: T) -> Self {
        Self {
            stream,
            closed: false,
        }
    }

    pub fn recv(&mut self) -> Result<Message, WebsocketError> {
        let message = Message::from_stream(&mut self.stream);

        if let Err(WebsocketError::ConnectionClosed) = message {
            self.closed = true;
        }

        message
    }

    /// Sends a message to the client.
    pub fn send(&mut self, message: Message) -> Result<(), WebsocketError> {
        self.stream
            .write_all(&message.to_bytes())
            .map_err(|_| WebsocketError::WriteError)
    }

    /// Returns a mutable reference to the underlying stream.
    pub fn inner(&mut self) -> &mut T {
        &mut self.stream
    }
}

impl<T> Drop for WebsocketStream<T>
where
    T: Read + Write,
{
    fn drop(&mut self) {
        if !self.closed {
            self.stream
                .write_all(Frame::new(Opcode::Close, Vec::new()).as_ref())
                .ok();
        }
    }
}