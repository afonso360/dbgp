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

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        ParseInt(::std::num::ParseIntError);
        Utf8(::std::str::Utf8Error);
        StringUtf8(::std::string::FromUtf8Error);
        Xml(::serde_xml_rs::Error);
    }

}

// TODO: Rename this
pub type AllPackets = Packet<PacketVariants>;

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PacketVariants {
    #[serde(rename = "init")]
    Init(Init),


    #[serde(rename = "response")]
    ResponseStatus(ResponseStatus),

    #[serde(rename = "response")]
    ResponseFeatureGet(ResponseFeatureGet),

    #[serde(rename = "response")]
    ResponseFeatureSet(ResponseFeatureSet),

    #[serde(rename = "response")]
    ResponseBreak(ResponseBreak),
}

pub mod init;
pub mod packet;
pub mod response;

pub use self::init::*;
pub use self::packet::*;
pub use self::response::*;
