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

/// Response packets
///
/// According to the standard all response packets can have error nodes inside them

use {TransactionId, SessionStatus, BreakReason, ErrorResponseString};

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
#[serde(rename = "response")]
pub struct ResponseFeatureGet {
     // TODO: xmlns parsing isn't working
     //pub xmlns: String,
     #[serde(deserialize_with = "::helpers::from_str")]
     pub transaction_id: TransactionId,
     pub command: String,

     pub feature_name: String,

     #[serde(deserialize_with = "::helpers::from_str_bool")]
     pub supported: bool,

     #[serde(rename = "$value")]
     pub data: ErrorResponseString,
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Deserialize, Serialize)]
#[serde(rename = "response")]
pub struct ResponseFeatureSet {
     // TODO: xmlns parsing isn't working
     //pub xmlns: String,
     #[serde(deserialize_with = "::helpers::from_str")]
     pub transaction_id: TransactionId,
     pub command: String,

     pub feature: String,

     #[serde(deserialize_with = "::helpers::from_str_bool")]
     pub success: bool,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Deserialize, Serialize)]
#[serde(rename = "response")]
pub struct ResponseStatus {
     // TODO: xmlns parsing isn't working
     //pub xmlns: String,
     #[serde(deserialize_with = "::helpers::from_str")]
     pub transaction_id: TransactionId,
     pub command: String,

     pub status: SessionStatus,
     pub reason: BreakReason,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Deserialize, Serialize)]
#[serde(rename = "response")]
pub struct ResponseBreak {
     // TODO: xmlns parsing isn't working
     //pub xmlns: String,
     #[serde(deserialize_with = "::helpers::from_str")]
     pub transaction_id: TransactionId,
     pub command: String,

     #[serde(rename = "$value")]
     pub data: ErrorResponseString,
}

#[cfg(test)]
mod tests {
    use serde_xml_rs;
    use ::*;
    use packets::response::*;

    #[test]
    fn deserialize_response_status_packet() {
            deserialize_test!(
                r##"
                <?xml version="1.0" encoding="UTF-8" ?>
			    <response xmlns="urn:debugger_protocol_v1"
                          transaction_id="792"
                          status="starting"
                          command="status"
                          reason="ok"/>
                "##,
                ResponseStatus {
                    //xmlns: Some(String::from("urn:debugger_protocol_v1")),
                    transaction_id: 792,
                    command: String::from("status"),
                    status: SessionStatus::Starting,
                    reason: BreakReason::Ok,
                }
            )
    }

    #[test]
    fn deserialize_response_feature_get_packet() {
            deserialize_test!(
                r##"
                <?xml version="1.0" encoding="UTF-8" ?>
                <response transaction_id="0"
                          feature_name="async"
                          command="feature_get"
                          supported="0"
                          xmlns="urn:debugger_protocol_v1">
                    <![CDATA[false]]>
                </response>
                "##,

                ResponseFeatureGet {
                    //xmlns: Some(String::from("urn:debugger_protocol_v1")),
                    transaction_id: 0,
                    command: String::from("feature_get"),
                    supported: false,
                    feature_name: String::from("async"),
                    data: ErrorResponseString::Ok(String::from("false")),
                }
            );
            deserialize_test!(
                r##"
                <?xml version="1.0" encoding="UTF-8" ?>
                <response transaction_id="0"
                          feature_name="language_name"
                          command="feature_get"
                          supported="1"
                          xmlns="urn:debugger_protocol_v1">
                    <![CDATA[Lua]]>
                </response>
                "##,

                ResponseFeatureGet {
                    //xmlns: Some(String::from("urn:debugger_protocol_v1")),
                    transaction_id: 0,
                    command: String::from("feature_get"),
                    supported: true,
                    feature_name: String::from("language_name"),
                    data: ErrorResponseString::Ok(String::from("Lua")),
                }
            );
    }

    #[test]
    fn deserialize_response_feature_set_packet() {
            deserialize_test!(
                r##"
                <?xml version="1.0" encoding="UTF-8" ?>
                <response feature="language_name"
                          success="0"
                          command="feature_set"
                          transaction_id="999"
                          xmlns="urn:debugger_protocol_v1" />
                "##,

                ResponseFeatureSet {
                    //xmlns: Some(String::from("urn:debugger_protocol_v1")),
                    transaction_id: 999,
                    command: String::from("feature_set"),
                    success: false,
                    feature: String::from("language_name"),
                }
            )
    }


    //Can all responses recieve error nodes as $value?
    //    Section 6.5:
    //        A debugger engine may need to relay error information back to the IDE in response to
    //        any command The debugger engine may add an error element as a child of the
    //        response element

    //Remove ResponseGeneric
    //Test breakpoint commands
    //separate responses into multiple files
    #[test]
    fn deserialize_response_break_packet() {
            deserialize_test!(
                r##"
                <?xml version="1.0" encoding="UTF-8" ?>
                <response command="break"
                          transaction_id="0"
                          xmlns="urn:debugger_protocol_v1">
                    <error code="998">
                        <message>
                            <![CDATA[attempt to index field 'previous_context']]>
                        </message>
                    </error>
                </response>
                "##,

                ResponseBreak {
                    //xmlns: Some(String::from("urn:debugger_protocol_v1")),
                    transaction_id: 999,
                    command: String::from("feature_set"),
                    data: ErrorResponseString::Err(PacketError{
                        code: 998,
                        data: vec![
                            Message::new("attempt to index field 'previous_context'")
                        ],
                    })

                }
            )
    }
}
