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

use super::{Command, Response};
use super::flag::Flag;
use xml::reader::XmlEvent;

response!(struct StdoutResponse {});

command!("stdout", struct Stdout {
    rediretion_type: RedirectionType: 'c'
}, StdoutResponse, |i: &Stdout, xml: XmlEvent| {
    StdoutResponse{}
});

response!(struct StderrResponse {});

command!("stderr", struct Stderr {
    rediretion_type: RedirectionType: 'c'
}, StderrResponse, |i: &Stderr, xml: XmlEvent| {
    StderrResponse{}
});

pub enum RedirectionType {
    /// stdout/stderr output goes to regular place, but not to Debug Client
    Disable,
    /// stdout/stderr output goes to both regular destination and Debug Client
    Copy,
    /// stdout/stderr output goes to Debug Client only.
    Redirect
}

impl Flag for RedirectionType {
    fn format_flag(&self, flag: char) -> String {
            format!("-{} {}", flag, match *self {
                RedirectionType::Disable => "0",
                RedirectionType::Copy => "1",
                RedirectionType::Redirect => "2",
            })
    }
}


