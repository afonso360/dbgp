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

response!(struct PropertyGetResponse {});

command!("property_get", struct PropertyGet {
    stack_depth:        Option<u32>: 'd',
    context_id:         Option<u32>: 'c',
    property_long_name: String:      'n',
    max_data:           Option<u32>: 'm',
    data_page:          Option<u32>: 'p',
    property_key:       Option<u32>: 'k'
}, PropertyGetResponse, |i: &PropertyGet, xml: XmlEvent| {
    PropertyGetResponse{}
});


response!(struct PropertySetResponse {});

command!("property_set", struct PropertySet {
    stack_depth:        Option<u32>:    'd',
    context_id:         Option<u32>:    'c',
    property_long_name: String:         'n',
    max_data:           Option<u32>:    'm',
    data_type:          Option<String>: 't',
    data_page:          Option<u32>:    'p',
    property_address:   Option<u32>:    'a'
}, PropertySetResponse, |i: &PropertySet, xml: XmlEvent| {
    PropertySetResponse{}
});


response!(struct PropertyValueResponse {});

command!("property_value", struct PropertyValue {
    stack_depth:        Option<u32>: 'd',
    context_id:         Option<u32>: 'c',
    property_long_name: String:      'n',
    max_data:           Option<u32>: 'm',
    data_page:          Option<u32>: 'p',
    property_address:   Option<u32>: 'a',
    property_key:       Option<u32>: 'k'
}, PropertyValueResponse, |i: &PropertyValue, xml: XmlEvent| {
    PropertyValueResponse{}
});
