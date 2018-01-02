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

//! This is mostly for types that fit inside Packets

pub mod errors;

pub use errors::*;

pub type TransactionId = u64;

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Deserialize, Serialize)]
#[allow(non_camel_case_types)] // TODO: Is there a better way to do this?
/// Enumerates all known protocol versions
/// Currently the only accepted version is 1.0
pub enum ProtocolVersion {
    #[serde(rename = "1.0")]
    V1_0,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BreakReason {
    Ok,
    Error,
    Aborted,
    Exception,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SessionStatus {
    Starting,
    Stopping,
    Stopped,
    Running,
    Break(BreakReason),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SessionType {
    Client,
    Server
}
