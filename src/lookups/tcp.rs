use std::rc::Rc;
use std::{net::SocketAddr, time::Duration};

use domain::base::Message;
use monoio::{
    io::{AsyncReadRentExt, AsyncWriteRentExt},
    net::TcpStream,
};

use crate::{errors::Error, iter::IpAddresses};

/// Query a nameserver for the given question, using the TCP protocol.
#[cold]

pub(crate) async fn query(
    id: u16,
    query: Rc<Vec<u8>>,
    nameserver: &SocketAddr,
    _attempts: u8,
    _timeout_duration: Duration,
    _udp_payload_size: u16,
) -> Result<Option<IpAddresses>, Error> {
    if query.len() > u16::MAX as usize {
        return Err(Error::QueryTooLarge {});
    }

    // Open the socket to the server.
    let mut socket = TcpStream::connect(nameserver).await?;

    // Write the length of the query.
    let len_bytes = Vec::from((query.len() as u16).to_be_bytes());
    let (result, mut len_bytes) = socket.write_all(len_bytes).await;
    let _ = result?;

    // Write the query.
    let (result, _) = socket.write_all(query).await;
    let _ = result?;

    // Read the length of the response.
    len_bytes.clear();
    let (result, len_bytes) = socket.read_exact(len_bytes).await;
    let _ = result?;

    let len = u16::from_be_bytes([len_bytes[0], len_bytes[1]]) as usize;

    // Initialize the heap buffer and return a pointer to it.
    let buf = vec![0; len];
    let (result, buf) = socket.read_exact(buf).await;
    let _ = result?;

    let message = Message::from_octets(buf)?;
    // Check the ID.
    if message.header().id() != id {
        return Ok(None);
    }
    let ip_addresses = IpAddresses::from(message);
    Ok(Some(ip_addresses))
}
