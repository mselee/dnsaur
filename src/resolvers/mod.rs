use std::{borrow::Borrow, net::IpAddr, str::FromStr, time::Duration};

use crate::{errors::Error, StubResolver};

mod hosts;
mod resolv;

impl StubResolver {
    pub async fn lookup<'a, B>(
        &'a self,
        host: impl AsRef<str> + Borrow<str> + 'a,
    ) -> Result<B, Error>
    where
        B: FromIterator<(IpAddr, Duration)>,
    {
        if let Ok(ip) = IpAddr::from_str(host.as_ref()) {
            return Ok(std::iter::once((ip, Duration::ZERO)).collect());
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
