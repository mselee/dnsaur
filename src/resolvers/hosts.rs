use std::{borrow::Borrow, net::IpAddr, time::Duration};

use crate::StubResolver;

impl StubResolver {
    pub(super) fn query_hosts<'a>(
        &'a self,
        host: impl AsRef<str> + Borrow<str> + 'a,
    ) -> impl Iterator<Item = (IpAddr, Duration)> + 'a {
        self.entries
            .iter()
            .filter(move |entry| entry.hosts.contains(host.as_ref()))
            .map(|entry| (entry.ip, Duration::ZERO))
    }
}
