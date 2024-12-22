# dnsaur

An asynchronous DNS stub resolver.

## Motivation
The [monoio](https://github.com/bytedance/monoio) async runtime does not ship with an asynchronous DNS resolver, and rather relies
on a threadpool to handle the blocking calls (like most other runtimes). This library aims to do the resolution in an async manner, including the file system access (using `io_uring`).

## Usage
```rust
use std::net::IpAddr;
use std::time::Duration;
use std::collections::BTreeSet;

use dnsaur::{StubResolver, Error};

#[monoio::main(driver = "iouring", enable_timer = true)]
async fn main() -> Result<(), Error> {
    let mut dns = dnsaur::StubResolver::load().await?;
    // pairs of (ip, ttl)
    let ips: Vec<(IpAddr, Duration)> = dns.lookup("example.com").await?;
    let ips = dns.lookup::<BTreeSet<_>>("example.com").await?;
    // reload
    let _ = dns.reload().await?;
    Ok(())
}
```

or if you want a global client instance, enable the feature `global`:
```rust
use std::net::IpAddr;
use std::time::Duration;
use std::collections::BTreeSet;

use dnsaur::Error;

#[monoio::main(driver = "iouring", enable_timer = true)]
async fn main() -> Result<(), Error> {
    // pairs of (ip, ttl)
    let ips: Vec<(IpAddr, Duration)> = dnsaur::lookup("example.com").await?;
    let ips = dnsaur::lookup::<BTreeSet<_>>("example.com").await?;
    // reload
    let _ = dnsaur::reload().await?;
    // autoreload
    monoio::spawn(async {
       loop {
           monoio::time::sleep(Duration::from_secs(5 * 60)).await;
           let _ = dnsaur::reload().await?;
       }
       Ok::<(), Error>(())
    });
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
| limitation   | glibc | dnsaur     |
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
