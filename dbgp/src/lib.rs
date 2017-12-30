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
extern crate bytes;
extern crate base64;
extern crate xml;

pub mod escape;
mod error_codes;
mod commands;
//mod transaction;

use std::io;

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
pub struct Session<R: io::Read, W: io::Write> {
    /// Read channel
    in_channel: R,

    /// Write channel
    out_channel: W,

    /// Represents the debugger status
    status: SessionStatus,

    session_type: SessionType,
}

impl<R: io::Read, W: io::Write> Session<R, W> {
    /// Creates a new session with the default parameters
    pub fn new(in_channel: R, out_channel: W, session_type: SessionType) -> Session<R, W> {
        Session {
            in_channel,
            out_channel,
            session_type,
            status: SessionStatus::Starting,
        }
    }
}
