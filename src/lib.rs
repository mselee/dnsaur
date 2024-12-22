#![forbid(future_incompatible)]
#![cfg_attr(not(feature = "global"), forbid(unsafe_code))]

mod addr;
mod errors;
mod lookups;
mod parser;
#[doc = include_str!("../README.md")]
pub mod readme;
mod resolvers;
use std::{
    collections::BTreeSet,
    net::{IpAddr, SocketAddr},
    time::Duration,
};

pub use errors::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct HostEntry {
    pub ip: IpAddr,
    pub hosts: BTreeSet<String>,
}

impl HostEntry {
    pub fn new(ip: IpAddr, hosts: impl Iterator<Item = String>) -> Self {
        Self {
            ip,
            hosts: hosts.collect(),
        }
    }
}

#[cfg(unix)]
pub struct StubResolver {
    entries: Vec<HostEntry>,
    search: Vec<String>,
    nameservers: Vec<SocketAddr>,
    timeout: Duration,
    ndots: u8,
    attempts: u8,
    rotate: bool,
    udp_payload_size: u16,
}

#[cfg(feature = "global")]
thread_local! {
    pub(crate) static GLOBAL: local_sync::OnceCell<StubResolver> = local_sync::OnceCell::new();
}

#[cfg(feature = "global")]
pub async fn lookup<'a, B>(
    host: impl AsRef<str> + std::borrow::Borrow<str> + 'a,
) -> Result<B, Error>
where
    B: FromIterator<(IpAddr, Duration)> + Sized,
{
    let global = GLOBAL.with(|global| unsafe {
        std::ptr::NonNull::new_unchecked(
            global as *const _ as *mut local_sync::OnceCell<StubResolver>,
        )
        .as_ref()
    });

    let dns: &StubResolver = global.get_or_try_init(|| StubResolver::load()).await?;
    dns.lookup(host).await
}

#[cfg(feature = "global")]
pub async fn reload() -> Result<(), Error> {
    let global = GLOBAL.with(|global| unsafe {
        std::ptr::NonNull::new_unchecked(
            global as *const _ as *mut local_sync::OnceCell<StubResolver>,
        )
        .as_mut()
    });

    if let Some(dns) = global.get_mut() {
        dns.reload().await
    } else {
        Ok(())
    }
}
