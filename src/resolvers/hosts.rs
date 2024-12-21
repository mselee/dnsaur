use std::{borrow::Borrow, net::IpAddr};

use crate::DnsResolver;

impl DnsResolver {
    pub(super) fn query_hosts<'a, S>(&'a self, host: S) -> impl Iterator<Item = IpAddr> + 'a
    where
        S: AsRef<str> + Borrow<str> + 'a,
    {
        self.entries
            .iter()
            .filter(move |entry| entry.hosts.contains(host.as_ref()))
            .map(|entry| entry.ip)
    }
}
