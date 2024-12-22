use std::{borrow::Borrow, net::IpAddr, str::FromStr};

use crate::{errors::Error, DnsResolver};

mod hosts;
mod resolv;

impl DnsResolver {
    pub async fn lookup<'a, S, B>(&'a self, host: S) -> Result<B, Error>
    where
        S: AsRef<str> + Borrow<str> + 'a,
        B: FromIterator<IpAddr>,
    {
        if let Ok(ip) = IpAddr::from_str(host.as_ref()) {
            return Ok(std::iter::once(ip).collect());
        }

        let mut count: u16 = 0;
        let addrs = self
            .query_hosts(host.as_ref())
            .inspect(|_| count += 1)
            .collect();

        if count != 0 {
            return Ok(addrs);
        }

        let addrs = self.query_resolv::<B>(host.as_ref()).await?;

        Ok(addrs)
    }
}
