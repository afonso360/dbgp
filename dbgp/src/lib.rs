/*
 * Copyright (c) the dbgp contributors. All rights reserved.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation. This file is also subject
 * to the Linking exception provided in the LICENSE file that
 * accompanied this code.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details (a copy is included in the LICENSE file that
 * accompanied this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 */

//#![deny(missing_docs)]
//#![deny(warnings)]
#![doc(test(attr(allow(unused_variables), deny(warnings))))]

//! This library implements the dbgp protocol

extern crate tokio_io;
extern crate tokio_core;
extern crate futures;

pub mod escape;

use futures::{Future, Stream};
use tokio_io::{io, AsyncRead};
use tokio_io::io::copy;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use std::net::{IpAddr, SocketAddr, Ipv4Addr};

enum Commands {
    feature_get,
    feature_set,
}

pub struct Session {
    core: Core,
    address: SocketAddr,
}

impl Session {
    pub fn new() -> Session {
        Session {
            core: Core::new().unwrap(),
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 9000),
        }
    }

    pub fn address(mut self, addr: IpAddr) -> Self {
        self.address.set_ip(addr);
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.address.set_port(port);
        self
    }

    pub fn run(mut self) {
        let handle = self.core.handle();
        let socket = TcpListener::bind(&self.address, &handle).unwrap();
        let server = socket.incoming().for_each(move |(socket, addr)| {
            let (rx, tx) = socket.split();
            let amt = copy(rx, tx);

            let msg = amt.then(move |result| {
                match result {
                    Ok((amt, _, _)) => println!("wrote {} bytes to {}", amt, addr),
                    Err(e) => println!("error on {}: {}", addr, e),
                }

                Ok(())
            });

            handle.spawn(msg);

            Ok(())
        });
        self.core.run(server).unwrap();
    }
}
