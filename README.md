# dnsur

An asynchronous DNS stub resolver.

## Motivation
The [monoio](https://github.com/bytedance/monoio) async runtime does not ship with an asynchronous DNS resolver, and rather relies
on a threadpool to handle the blocking calls (like most other runtimes). This library aims to do the resolution in an async manner, including the file system access (using `io_uring`).

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
Mozilla Public License Version 2.0. See the [LICENSE](LICENSE) file for details.

## Acknowledgement
This library is inspired by [async-dns](https://github.com/notgull/async-dns)
