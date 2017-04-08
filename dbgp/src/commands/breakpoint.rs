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
use base64;

response!(struct BreakpointSetResponse {});

command!("breakpoint_set", struct BreakpointSet {
    hit_value: u32:                     'h',
    btype:     BreakpointType:          't',
    state:     Option<BreakpointState>: 's',
    temporary: bool:                    't'
}, BreakpointSetResponse, |i: &BreakpointSet, xml: XmlEvent| {
    BreakpointSetResponse{}
});

response!(struct BreakpointGetResponse {});

command!("breakpoint_get", struct BreakpointGet {
    breakpoint_id: u32: 'd'
}, BreakpointGetResponse, |i: &BreakpointGet, xml: XmlEvent| {
    BreakpointGetResponse{}
});

response!(struct BreakpointRemoveResponse {});

command!("breakpoint_remove", struct BreakpointRemove {
    breakpoint_id: u32: 'd'
}, BreakpointRemoveResponse, |i: &BreakpointRemove, xml: XmlEvent| {
    BreakpointRemoveResponse{}
});

response!(struct BreakpointUpdateResponse {});

command!("breakpoint_update", struct BreakpointUpdate {
    breakpoint_id: u32:                     'd',
    state:         Option<BreakpointState>: 's',
    lineno:        Option<u32>:             'n',
    hit_value:     Option<u32>:             'h',
    hit_condition: Option<String>:          'o'
}, BreakpointUpdateResponse, |i: &BreakpointUpdate, xml: XmlEvent| {
    BreakpointUpdateResponse{}
});

response!(struct BreakpointListResponse {});

command!("breakpoint_list",
         struct BreakpointList {},
         BreakpointListResponse,
         |i: &BreakpointList, xml: XmlEvent| {
    BreakpointListResponse{}
});

pub enum BreakpointType {
    Line{ filename: String, lineno: u32 },
    Call{ function: String },
    Return{ function: String },
    Exception{ exception: String },
    Conditional{ expression: String, filename: String },
    Watch{ expression: String },
}

impl Flag for BreakpointType {
    fn format_flag(&self, flag: char) -> String {
        format!("-{} {}", flag, match *self {
            BreakpointType::Line{ filename: ref f, lineno: ref l } =>
                format!("line -f {} -l {}", f, l),
            BreakpointType::Call{ function: ref m } =>
                format!("call -m {}", m),
            BreakpointType::Return{ function: ref m } =>
                format!("return -m {}", m),
            BreakpointType::Exception{ exception: ref x } =>
                format!("exception -x {}", x),
            BreakpointType::Conditional{ expression: ref exp, filename: ref f } =>
                format!("conditional -f {} -- {}", f, base64::encode(exp.as_bytes())),
            BreakpointType::Watch{ expression: ref exp } =>
                format!("watch -- {}", base64::encode(exp.as_bytes())),
        })
    }
}


pub enum BreakpointState {
    Enabled,
    Disabled
}


impl Flag for BreakpointState {
    fn format_flag(&self, flag: char) -> String {
        match *self {
            BreakpointState::Enabled => format!("-{} enabled", flag),
            BreakpointState::Disabled => format!("-{} disabled", flag),
        }
    }
}
