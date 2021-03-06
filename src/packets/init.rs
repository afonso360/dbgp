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
use ProtocolVersion;

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Deserialize, Serialize)]
#[serde(rename = "init")]
pub struct Init {
    pub appid: String,
    pub idekey: String,
    pub session: String,
    pub thread: String,
    pub parent: String,
    pub language: String,
    pub protocol_version: ProtocolVersion,

    #[serde(with = "url_serde")]
    pub fileuri: Url,
}

#[cfg(test)]
mod tests {
    use ::*;
    use super::Init;
    use url::Url;

    #[test]
    fn deserialize_init_packet_simple() {
        deserialize_test!(
            r##"
            <?xml version="1.0" encoding="UTF-8"?>
            <init appid="APPID"
                  idekey="IDE_KEY"
                  session="DBGP_COOKIE"
                  thread="THREAD_ID"
                  parent="PARENT_APPID"
                  language="LANGUAGE_NAME"
                  protocol_version="1.0"
                  fileuri="file://path/to/file" />
            "##,
            Init {
                appid: String::from("APPID"),
                idekey: String::from("IDE_KEY"),
                session: String::from("DBGP_COOKIE"),
                thread: String::from("THREAD_ID"),
                parent: String::from("PARENT_APPID"),
                language: String::from("LANGUAGE_NAME"),
                protocol_version: ProtocolVersion::V1_0,
                fileuri: Url::parse("file://path/to/file").unwrap(),
            }
        )
    }

    // TODO: Parse with child elements
}
