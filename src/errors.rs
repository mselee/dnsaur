//
// Copyright (c) 2024 Mohamed Seleem <oss@mselee.com>.
//
// This file is part of dnsaur.
// See https://github.com/mselee/dnsaur for further info.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//

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
