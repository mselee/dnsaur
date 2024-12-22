# dnsur

An asynchronous DNS stub resolver.

## Motivation
The [monoio](https://github.com/bytedance/monoio) async runtime does not ship with an asynchronous DNS resolver, and rather relies
on a threadpool to handle the blocking calls (like most other runtimes). This library aims to do the resolution in an async manner, including the file system access (using `io_uring`).

## Usage
```rust
use std::net::IpAddr;
use dnsur::{DnsResolver, Error};

#[monoio::main(driver = "iouring", enable_timer = true)]
async fn main() -> Result<(), Error> {
    let dns = dnsur::DnsResolver::parse().await?;
    let ips: Vec<IpAddr> = dns.lookup("google.com").await?;
    dbg!(ips);
    Ok(())
}
```

or if you want a global client instance, enable the feature `global`:
```rust
use std::net::IpAddr;
use dnsur::Error;

#[monoio::main(driver = "iouring", enable_timer = true)]
async fn main() -> Result<(), Error> {
    let ips: Vec<IpAddr> = dnsur::lookup("google.com").await?;
    dbg!(ips);
    Ok(())
}
```

## Details
- `/etc/hosts` and `/etc/resolv.conf` are parsed to build the configuration.
- The entries in the `hosts` are tried first, and if not present, the `nameservers` from `resolv.conf` will be queried.
- Querying of the nameservers is done sequentially (i.e. we query the second nameserver only if the first one has failed).
- A and AAAA records are queried concurrently.
- Default UDP buffer size is 1232 bytes.

## Non-standard behavior
| limitation   | glibc | dnsur      |
| ------------ | ----- | ---------- |
| `nameserver` | 3     | unlimited  |
| `timeout`    | 30    | `u8::MAX` |
| `ndots`      | 15    | `u8::MAX`  |
| `attempts`   | 5     | `u8::MAX`  |

## TODO
- [ ] Support for `rotate`

## Status
Hic Sunt Dracones

## License
Mozilla Public License Version 2.0. See the [LICENSE](./LICENSE) file for details.

## Acknowledgement
This library is inspired by [async-dns](https://github.com/notgull/async-dns)
