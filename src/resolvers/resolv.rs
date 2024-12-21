use std::{
    net::{IpAddr, SocketAddr},
    rc::Rc,
    str::FromStr,
    time::Duration,
};

use domain::base::{
    iana::Class, name::UncertainName, wire::Composer, MessageBuilder, Question, Rtype,
    StaticCompressor, ToName,
};

use crate::{errors::Error, iter::IpAddresses, DnsResolver};

impl DnsResolver {
    pub(super) async fn query_resolv(&self, name: &str) -> Result<Vec<IpAddr>, Error> {
        self.dns_with_search(name).await
    }

    async fn dns_with_search(&self, name: &str) -> Result<Vec<IpAddr>, Error> {
        // See if we should just use global scope.
        let num_dots = memchr::Memchr::new(b'.', name.as_bytes()).count();
        let global_scope = num_dots >= self.ndots as usize || name.ends_with(".");
        let name = name.trim_end_matches('.');

        if global_scope {
            let it = self.search.iter();
            // Try the name with the search domains.
            let mut host = String::from(name);
            host.push('.');
            let size = host.len();
            for search in it {
                // Try the name with the search domains.
                host.truncate(size);
                host.push_str(search.trim_start_matches('.'));

                let name = UncertainName::<Vec<u8>>::from_str(&host)?.into_absolute()?;
                if let Ok(addrs) = self.dns_lookup(name).await {
                    if !addrs.is_empty() {
                        return Ok(addrs);
                    }
                }
            }
        }

        let name = UncertainName::<Vec<u8>>::from_str(name)?.into_absolute()?;
        // Preform a DNS search on just the name.
        self.dns_lookup(name).await
    }

    /// Preform a manual lookup for the name.
    async fn dns_lookup(&self, name: impl ToName) -> Result<Vec<IpAddr>, Error> {
        let it = self.nameservers.iter();
        for nameserver in it {
            if let Ok(addrs) = self.query_name_and_nameserver(&name, nameserver).await {
                return Ok(addrs);
            }
        }
        Ok(vec![])
    }

    /// Poll for the name on the given nameserver.
    async fn query_name_and_nameserver<B>(
        &self,
        name: impl ToName,
        nameserver: &SocketAddr,
    ) -> Result<B, Error>
    where
        B: FromIterator<IpAddr>,
    {
        // Try to poll for an IPv4 address first.
        let ipv4 = query_question_and_nameserver(
            Question::new(&name, Rtype::A, Class::IN),
            nameserver,
            self.attempts,
            self.timeout,
            self.udp_payload_size,
        );

        let ipv6 = query_question_and_nameserver(
            Question::new(&name, Rtype::AAAA, Class::IN),
            nameserver,
            self.attempts,
            self.timeout,
            self.udp_payload_size,
        );

        let (ipv4, ipv6) = monoio::join!(ipv4, ipv6);
        let ipv4 = ipv4?;
        let ipv6 = ipv6?;
        let addrs = ipv4.iter().chain(ipv6.iter()).flat_map(|x| x.iter());
        let addrs = FromIterator::from_iter(addrs);
        Ok(addrs)
    }
}

fn create_message<T: Composer + Default>(
    id: u16,
    question: Question<impl ToName>,
    udp_payload_size: u16,
) -> Result<StaticCompressor<T>, Error> {
    // Create the DNS query.
    let mut message = MessageBuilder::from_target(StaticCompressor::new(Default::default()))
        .map_err(|_| Error::AppendError {})?;
    message.header_mut().set_rd(true);
    message.header_mut().set_id(id);
    let mut message = message.question();
    message.push(question)?;
    let mut message = message.additional();
    message.opt(|opt| {
        opt.set_udp_payload_size(udp_payload_size);
        Ok(())
    })?;
    Ok(message.finish())
}

/// Poll for a DNS response on the given nameserver.
async fn query_question_and_nameserver(
    question: Question<impl ToName>,
    nameserver: &SocketAddr,
    attempts: u8,
    timeout_duration: Duration,
    udp_payload_size: u16,
) -> Result<Option<IpAddresses>, Error> {
    let id = fastrand::u16(..);
    let message = create_message::<Vec<u8>>(id, question, udp_payload_size)?;
    let data: Rc<Vec<u8>> = Rc::from(message.into_target());

    // The query may be too large, so we need to use TCP.
    if data.len() <= udp_payload_size as usize {
        if let Some(addrs) = crate::lookup::udp::query(
            id,
            data.clone(),
            nameserver,
            attempts,
            timeout_duration,
            udp_payload_size,
        )
        .await?
        {
            return Ok(Some(addrs));
        }
    }

    // We were unable to complete the query over UDP, use TCP instead.
    crate::lookup::tcp::query(
        id,
        data,
        nameserver,
        attempts,
        timeout_duration,
        udp_payload_size,
    )
    .await
}
