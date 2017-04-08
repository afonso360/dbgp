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

response!(struct SpawnpointSetResponse {});

command!("spawnpoint_set", struct SpawnpointSet {
    filename: Option<String>:          'f',
    lineno:   Option<u32>:             'n',
    state:    Option<SpawnpointState>: 's'
}, SpawnpointSetResponse, |i: &SpawnpointSet, xml: XmlEvent| {
    SpawnpointSetResponse{}
});

response!(struct SpawnpointGetResponse {});

command!("spawnpoint_get", struct SpawnpointGet {
    id: u32: 'i'
}, SpawnpointGetResponse, |i: &SpawnpointGet, xml: XmlEvent| {
    SpawnpointGetResponse{}
});


response!(struct SpawnpointUpdateResponse {});

command!("spawnpoint_update", struct SpawnpointUpdate {
    lineno: Option<u32>:             'n',
    state:  Option<SpawnpointState>: 's'
}, SpawnpointUpdateResponse, |i: &SpawnpointUpdate, xml: XmlEvent| {
    SpawnpointUpdateResponse{}
});


response!(struct SpawnpointRemoveResponse {});

command!("spawnpoint_remove", struct SpawnpointRemove {
    id: u32: 'i'
}, SpawnpointRemoveResponse, |i: &SpawnpointRemove, xml: XmlEvent| {
    SpawnpointRemoveResponse{}
});


response!(struct SpawnpointListResponse {});

command!("spawnpoint_list",
         struct SpawnpointList {},
         SpawnpointListResponse,
         |i: &SpawnpointList, xml: XmlEvent| {
    SpawnpointListResponse{}
});


pub enum SpawnpointState {
    Enabled,
    Disabled
}

impl Flag for SpawnpointState {
    fn format_flag(&self, flag: char) -> String {
        match *self {
            SpawnpointState::Enabled => format!("-{} enabled", flag),
            SpawnpointState::Disabled => format!("-{} disabled", flag),
        }
    }
}
