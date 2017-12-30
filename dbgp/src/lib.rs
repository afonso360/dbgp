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

#[macro_use]
extern crate lazy_static;
extern crate bytes;
extern crate base64;
extern crate xml;

pub mod escape;
mod error_codes;
mod commands;
//mod transaction;

use std::io;
use std::net::{IpAddr, SocketAddr, Ipv4Addr};

lazy_static! {
    static ref DEFAULT_IP_ADDR: Ipv4Addr = {
        Ipv4Addr::new(127,0,0,1)
    };
}

static DEFAULT_PORT: u16 = 9000;

enum BreakReason {
    Ok,
    Error,
    Aborted,
    Exception,
}

enum SessionStatus {
    Starting,
    Stopping,
    Stopped,
    Running,
    Break(BreakReason),
}

pub enum SessionType {
    Client,
    Server
}

/// Represents a session with a debugger
pub struct Session {
    /// Address to listen to
    address: SocketAddr,

    /// Represents the debugger status, None if there is no connection yet
    status: Option<SessionStatus>,

    session_type: SessionType,
}

impl Session {
    /// Creates a new session with the default parameters
    pub fn new(address: SocketAddr, session_type: SessionType) -> Session {
        Session {
            address: address,
            session_type: session_type,
            status: None,
        }
    }
}


struct Dbgp;

impl Dbgp {
    pub fn connect_ssl(address: SocketAddr) -> Session {
        unimplemented!();
    }

    pub fn connect(address: SocketAddr) -> Result<Session, io::Error> {
        unimplemented!();
        //    let ret = TcpClient::new(DbgpProto)
        //        .connect(address, handle)
        //        .map(|client_proxy| {
        //            Session::new(address, SessionType::Client)
        //        });

        //    Box::new(ret)
    }

    /// This method will block the current thread until the server is shut down.
    pub fn serve_ssl(address: SocketAddr) {
        unimplemented!();
    }

    /// This method will block the current thread until the server is shut down.
    pub fn serve(address: SocketAddr) {
        unimplemented!();
        //TcpServer::new(DbgpProto, address)
        //    .serve(|h: &Handle| {
        //        Session::new(address, SessionType::Server, h)
        //    })
    }
}
