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
extern crate tokio_service;
extern crate tokio_proto;
extern crate futures;
extern crate bytes;

pub mod escape;
mod codec;
mod protocol;

use tokio_service::Service;
use futures::{future, Future, BoxFuture};
use std::io;
use tokio_proto::TcpServer;
use std::net::{IpAddr, SocketAddr, Ipv4Addr};
use protocol::DbgpProto;

//enum Commands {
//    feature_get,
//    feature_set,
//}

pub struct Session {
    address: SocketAddr,
}

impl Session {
    pub fn new() -> Session {
        Session {
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

    pub fn run(self) {
        let server = TcpServer::new(DbgpProto, self.address);

        server.serve(|| Ok(Echo));
    }
}

pub struct Echo;

impl Service for Echo {
    // These types must match the corresponding protocol types:
    type Request = String;
    type Response = String;

    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;

    // The future for computing the response; box it for simplicity.
    type Future = BoxFuture<Self::Response, Self::Error>;

    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        // In this case, the response is immediate.
        future::ok(req).boxed()
    }
}
