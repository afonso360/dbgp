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

use url_serde;
use url::Url;
use {SessionStatus, BreakReason};

// serde_xml_rs fails when parsing Untagged enums, so i'm going to duplicate all fields
// Fix it
//
// #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Deserialize, Serialize)]
// #[serde(rename = "response")]
// pub struct Response {
//     pub xmlns: Option<String>,
//     pub transaction_id: String,
//     pub command: String,
// }

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Deserialize, Serialize)]
#[serde(rename = "supprted")]
pub enum Supported {
    #[serde(rename = "1")]
    Yes,

    #[serde(rename = "0")]
    No,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Deserialize, Serialize)]
#[serde(rename = "response")]
pub struct ResponseFeatureGet {
     pub xmlns: Option<String>,
     pub transaction_id: String,
     pub command: String,

     pub feature_name: String,
     pub supported: Supported,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Deserialize, Serialize)]
#[serde(rename = "response")]
pub struct ResponseStatus {
     pub xmlns: Option<String>,
     pub transaction_id: String,
     pub command: String,

     pub status: SessionStatus,
     pub reason: BreakReason,
}

#[cfg(test)]
mod tests {
    use packets::Init;

    use serde_xml_rs;
    use url::Url;

    #[test]
    #[ignore]
    fn deserialize_response_packet_simple() {
    }

    // TODO: Parse with child elements

}
