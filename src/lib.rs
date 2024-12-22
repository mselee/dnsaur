#![forbid(future_incompatible)]
#![cfg_attr(not(feature = "global"), forbid(unsafe_code))]

mod errors;
mod iter;
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

#[cfg(feature = "global")]
use local_sync::OnceCell;

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
pub struct DnsResolver {
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
    pub(crate) static GLOBAL: OnceCell<DnsResolver> = OnceCell::new();
}

#[cfg(feature = "global")]
pub async fn lookup<'a, S, B>(host: S) -> Result<B, Error>
where
    S: AsRef<str> + std::borrow::Borrow<str> + 'a,
    B: FromIterator<IpAddr> + Sized,
{
    let global = GLOBAL.with(|global| unsafe {
        std::ptr::NonNull::new_unchecked(global as *const _ as *mut OnceCell<DnsResolver>).as_ref()
    });

    let dns: &DnsResolver = global.get_or_try_init(|| DnsResolver::parse()).await?;
    dns.lookup(host).await
}
