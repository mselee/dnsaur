use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(context(false))]
    TimeoutError {
        source: monoio::time::error::Elapsed,
    },
    #[snafu(context(false))]
    FileSystemError {
        source: std::io::Error,
    },
    #[snafu(context(false))]
    IpAddrParseError {
        source: std::net::AddrParseError,
    },
    #[snafu(context(false))]
    Utf8StrError {
        source: std::str::Utf8Error,
    },
    #[snafu(context(false))]
    Utf8StringError {
        source: std::string::FromUtf8Error,
    },
    #[snafu(context(false))]
    Utf8ByteStringError {
        source: bstr::Utf8Error,
    },
    #[snafu(context(false))]
    ShortMessage {
        source: domain::base::message::ShortMessage,
    },
    #[snafu(context(false))]
    MessagePushError {
        source: domain::base::message_builder::PushError,
    },
    #[snafu(context(false))]
    NameError {
        source: domain::base::name::NameError,
    },
    #[snafu(context(false))]
    DomainNameParseError {
        source: domain::base::name::FromStrError,
    },
    #[snafu(context(false))]
    DomainNamePushError {
        source: domain::base::name::PushError,
    },
    AppendError {},
    QueryTooLarge {},
    InvalidMessageID {
        expected: u16,
        found: u16,
    },
}
