use std::{
    net::{Ipv4Addr, Ipv6Addr, SocketAddr},
    rc::Rc,
    time::Duration,
};

use domain::base::Message;
use monoio::{io::Canceller, net::udp::UdpSocket};

use crate::{addr::IpAddresses, errors::Error};

/// Query a nameserver for the given question, using the UDP protocol.
///
/// Returns `None` if the UDP query failed and TCP should be used instead.
pub(crate) async fn query(
    id: u16,
    query: Rc<Vec<u8>>,
    nameserver: &SocketAddr,
    attempts: u8,
    timeout_duration: Duration,
    udp_payload_size: u16,
) -> Result<Option<IpAddresses>, Error> {
    // Write the query to the nameserver address.
    let bind_address: SocketAddr = match nameserver.ip() {
        std::net::IpAddr::V4(_) => (Ipv4Addr::UNSPECIFIED, 0).into(),
        std::net::IpAddr::V6(_) => (Ipv6Addr::UNSPECIFIED, 0).into(),
    };
    let socket = UdpSocket::bind(bind_address)?;

    async fn send(
        socket: &UdpSocket,
        buf: Rc<Vec<u8>>,
        server: SocketAddr,
    ) -> Result<usize, std::io::Error> {
        socket.send_to(buf, server).await.0
    }

    async fn recv(
        socket: &UdpSocket,
        buf: Vec<u8>,
        t: Duration,
    ) -> (Result<usize, std::io::Error>, Vec<u8>) {
        let canceller = Canceller::new();
        let handle = canceller.handle();
        let cancel_io = async move {
            monoio::time::sleep(t).await;
            canceller.cancel();
        };
        monoio::spawn(cancel_io);
        socket.cancelable_recv(buf, handle).await
    }

    for idx in 1..=attempts {
        let buf = Vec::with_capacity(udp_payload_size as usize);
        let result = send(&socket, query.clone(), *nameserver).await;
        if let Err(err) = result {
            if idx < attempts {
                continue;
            }
            return Err(Error::from(err));
        }
        let (result, buf) = recv(&socket, buf, timeout_duration).await;
        if let Err(err) = result {
            if idx < attempts {
                continue;
            }
            return Err(Error::from(err));
        };

        let message = Message::from_octets(buf)?;
        let header = message.header();

        // Check the ID.
        if header.id() != id {
            continue;
        }

        // Check truncation
        if header.tc() {
            return Ok(None);
        }

        let ip_addresses = IpAddresses::from(message);
        return Ok(Some(ip_addresses));
    }

    // We did not receive a response.
    Ok(None)
}
