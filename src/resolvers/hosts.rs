//
// Copyright (c) 2024 Mohamed Seleem <oss@mselee.com>.
//
// This file is part of dnsaur.
// See https://github.com/mselee/dnsaur for further info.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//

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
