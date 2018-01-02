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

use ::*;

// TODO: This would be better, but we cant use #[serde(untagged)] on it
// pub type ErrorResponse<T> = Result<T, PacketError>;

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Deserialize, Serialize)]
// TODO: Make this generic
#[serde(untagged)]
pub enum ErrorResponseString {
    Ok(String),
    Err(PacketError),
}

// TODO: Is this a good name?
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Deserialize, Serialize)]
#[serde(rename = "error")]
pub struct PacketError {
     #[serde(deserialize_with = "::helpers::from_str")]
    pub code: u16,

    // TODO: It shouldn't just be string, we can also have cusetom xml nodes
    // as a response, however, currently the plan is to just pass those nodes
    // into the client of this library as a string, so if this captures
    // the xml info as text, leave it
    #[serde(rename = "$value")]
    pub data: Vec<Message>,
}

trait DbgpError {
    /// Should provide a reason for the error
    // TODO: Should we return a static string?
    fn reason(&self) -> String;
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_error_node() {
            deserialize_test!(
                r##"
                <?xml version="1.0" encoding="UTF-8" ?>
                <error code="998">
                    <message>
                        <![CDATA[attempt to index field 'previous_context']]>
                    </message>
                </error>
                "##,

                PacketError {
                    code: 998,
                    data: vec![
                        Message::new("attempt to index field 'previous_context'"),
                    ],
                }
            )
    }

}

// TODO: Figure out a way to provide this, thats extensible to users
// of this library that want to extend to their own errors and reasons
//
// #[allow(unused_doc_comment)]
// pub static ERROR_CODES: [ErrorCode; 22] = [
//
//
//     // -------------- 000 Command parsing errors --------------
//     ErrorCode {
//         id: 000,
//         message: "no error",
//     },
//     ErrorCode {
//         id: 001,
//         message: "parse error in command",
//     },
//     ErrorCode {
//         id: 002,
//         message: "duplicate arguments in command",
//     },
//
//     /// missing a required option, invalid value for a
//     /// passed option, not supported feature
//     ErrorCode {
//         id: 003,
//         message: "invalid options",
//     },
//
//     ErrorCode {
//         id: 004,
//         message: "Unimplemented command",
//     },
//
//     /// Is used for async commands. For instance
//     /// if the engine is in state "run" then only "break" and "status"
//     /// are available
//     ErrorCode {
//         id: 005,
//         message: "Command not available",
//     },
//
//     // -------------- 100 File related errors --------------
//
//     /// as a reply to a "source" command if the
//     /// requested source file can't be opened
//     ErrorCode {
//         id: 100,
//         message: "can not open file",
//     },
//     ErrorCode {
//         id: 101,
//         message: "stream redirect failed",
//     },
//
//     // -------------- 200 Breakpoint, or code flow errors --------------
//
//     /// for some reason the breakpoint could not be set due
//     /// to problems registering it
//     ErrorCode {
//         id: 200,
//         message: "breakpoint could not be set",
//     },
//     /// for example I don't support 'watch' yet and thus return this error
//     ErrorCode {
//         id: 201,
//         message: "breakpoint type not supported",
//     },
//
//     /// the IDE tried to set a breakpoint on a
//     /// line that does not exist in the file (ie "line 0" or lines
//     /// past the end of the file)
//     ErrorCode {
//         id: 202,
//         message: "invalid breakpoint",
//     },
//     ///the IDE tried to set a breakpoint
//     ///on a line which does not have any executable code. The
//     ///debugger engine is NOT required to return this type if it
//     ///is impossible to determine if there is code on a given
//     ///location. (For example, in the PHP debugger backend this
//     ///will only be returned in some special cases where the current
//     ///scope falls into the scope of the breakpoint to be set)
//     ErrorCode {
//         id: 203,
//         message: "no code on breakpoint line",
//     },
//     /// using an unsupported breakpoint state was attempted
//     ErrorCode {
//         id: 204,
//         message: "Invalid breakpoint state",
//     },
//
//     /// used in breakpoint_get etc. to show that there is no
//     /// breakpoint with the given ID
//     ErrorCode {
//         id: 205,
//         message: "No such breakpoint",
//     },
//     /// use from eval() (or perhaps property_get for a full name get)
//     ErrorCode {
//         id: 206,
//         message: "Error evaluating code",
//     },
//
//     /// the expression used for a no, message:eval() was invalid },
//     ErrorCode {
//         id: 207,
//         message: "Invalid expression",
//     },
//
//     // -------------- 300 Data errors --------------
//
//     /// when the requested property to get did
//     /// not exist, this is NOT used for an existing but uninitialized
//     /// property, which just gets the type "uninitialised" (See:
//     /// PreferredTypeNames)
//     ErrorCode {
//         id: 300,
//         message: "Can not get property",
//     },
//
//     ///  the, message:d stack depth parameter did not },
//     ///  exist (ie, there were less stack elements than the number
//     ///  requested) or the parameter was < 0
//     ErrorCode {
//         id: 301,
//         message: "Stack depth invalid",
//     },
//
//     /// an non existing context was requested
//     ErrorCode {
//         id: 302,
//         message: "Context invalid",
//     },
//
//     // -------------- 900 Protocol errors --------------
//     ErrorCode {
//         id: 900,
//         message: "Encoding not supported",
//     },
//     ErrorCode {
//         id: 998,
//         message: "An internal exception in the debugger occurred",
//     },
//     ErrorCode {
//         id: 999,
//         message: "Unknown error",
//     }
// ];

