use std::str::FromStr;
use std::time::Duration;

use bstr::ByteSlice;
use monoio::fs::read;

use crate::errors::Error;
use crate::{DnsResolver, HostEntry};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

const NAMESERVER: &[u8] = "nameserver".as_bytes();
const OPTIONS: &[u8] = "options".as_bytes();
const OPTION_NDOTS: &[u8] = "ndots".as_bytes();
const OPTION_TIMEOUT: &[u8] = "timeout".as_bytes();
const OPTION_ATTEMPTS: &[u8] = "attempts".as_bytes();
const OPTION_ROTATE: &[u8] = "rotate".as_bytes();
const OPTION_EDNS0: &[u8] = "edns0".as_bytes();
const SEARCH: &[u8] = "search".as_bytes();
const DEFAULT_NAMESERVER_IPV4: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 53);
const DEFAULT_NAMESERVER_IPV6: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 53);

impl DnsResolver {
    async fn parse_hosts(&mut self) -> Result<(), Error> {
        let content = read("/etc/hosts").await?;
        for line in content.lines() {
            let mut it = line.fields().take_while(|field| !field.starts_with(b"#"));
            if let Some(ip) = it.next() {
                let ip = ip.to_str()?;
                let ip = IpAddr::from_str(ip)?;
                let hosts = it
                    .map(|host| String::from_utf8(host.to_owned()))
                    .filter_map(|host| host.ok());
                let entry = HostEntry::new(ip, hosts);
                self.entries.push(entry);
            } else {
                continue;
            };
        }
        Ok(())
    }

    async fn parse_resolv(&mut self, udp_payload_size: Option<u16>) -> Result<(), Error> {
        let content = read("/etc/resolv.conf").await?;
        for line in content.lines() {
            let mut it = line.fields().take_while(|field| !field.starts_with(b"#"));
            match it.next() {
                Some(SEARCH) => {
                    let it = it.filter_map(|x| x.to_str().ok()).map(|x| x.to_owned());
                    self.search = it.collect();
                }
                Some(NAMESERVER) => {
                    if let Some(ip) = it.next() {
                        let ip = ip.to_str()?;
                        let ip = IpAddr::from_str(ip)?;
                        let addr = SocketAddr::new(ip, 53);
                        self.nameservers.push(addr);
                    }
                }
                Some(OPTIONS) => {
                    for field in it {
                        if OPTION_EDNS0 == field {
                            self.udp_payload_size = udp_payload_size.unwrap_or(1232);
                        } else if let Some((key, value)) = field.split_once_str(":") {
                            match key {
                                OPTION_NDOTS => self.ndots = value[0],
                                OPTION_TIMEOUT => {
                                    self.timeout = Duration::from_secs(value[0] as u64)
                                }
                                OPTION_ATTEMPTS => self.attempts = value[0],
                                OPTION_ROTATE => self.rotate = true,
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        if self.nameservers.is_empty() {
            self.nameservers.push(DEFAULT_NAMESERVER_IPV4);
            self.nameservers.push(DEFAULT_NAMESERVER_IPV6);
        }
        Ok(())
    }

    pub async fn parse() -> Result<Self, Error> {
        let mut this = Self {
            entries: Vec::default(),
            search: Vec::default(),
            nameservers: Vec::default(),
            ndots: 1,
            timeout: Duration::from_secs(5),
            attempts: 2,
            rotate: false,
            udp_payload_size: 512,
        };
        this.parse_hosts().await?;
        this.parse_resolv(None).await?;
        Ok(this)
    }
}
