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

use super::super::escape;
use base64;

// TODO: Refactor this once the bug has been fixed
// We could use Specialization here but rust has a bug
// https://github.com/rust-lang/rust/issues/41140

pub trait Flag {
    fn format_flag(&self, flag: char) -> String;
}

impl Flag for u32 {
    fn format_flag(&self, flag: char) -> String {
        format!("-{} {}", flag, *self)
    }
}

impl<T> Flag for Option<T>
    where T: Flag {
    fn format_flag(&self, flag: char) -> String {
        match *self {
            Some(ref s) => s.format_flag(flag),
            None => "".to_string(),
        }
    }
}


impl Flag for String {
    fn format_flag(&self, flag: char) -> String {
        format!("-{} {}", flag, escape::escape(self.clone()))
    }
}



impl Flag for bool {
    fn format_flag(&self, flag: char) -> String {
        match *self {
            true => format!("-{} 1", flag),
            false => format!("-{} 0", flag),
        }
    }
}

impl Flag for [u8] {
    fn format_flag(&self, flag: char) -> String {
        format!("-{} {}", flag, base64::encode(self))
    }
}







//impl Flag for RedirectionType {
//    fn format_flag(&self, flag: char) -> String {
//        match *self {
//            RedirectionType::Disable => format!("-{} 0", flag),
//            RedirectionType::Copy => format!("-{} 1", flag),
//            RedirectionType::Redirect => format!("-{} 2", flag),
//        }
//    }
//}
//
//impl Flag for BreakpointState {
//    fn format_flag(&self, flag: char) -> String {
//        match *self {
//            BreakpointState::Enabled => format!("-{} enabled", flag),
//            BreakpointState::Disabled => format!("-{} disabled", flag),
//        }
//    }
//}
//
//
//impl Flag for BreakpointType {
//    fn format_flag(&self, flag: char) -> String {
//        format!("-{} {}", flag, match *self {
//            BreakpointType::Line{ filename: ref f, lineno: ref l } =>
//                format!("line -f {} -l {}", f, l),
//            BreakpointType::Call{ function: ref m } =>
//                format!("call -m {}", m),
//            BreakpointType::Return{ function: ref m } =>
//                format!("return -m {}", m),
//            BreakpointType::Exception{ exception: ref x } =>
//                format!("exception -x {}", x),
//            BreakpointType::Conditional{ expression: ref exp, filename: ref f } =>
//                format!("conditional -f {} -- {}", f, base64::encode(exp.as_bytes())),
//            BreakpointType::Watch{ expression: ref exp } =>
//                format!("watch -- {}", base64::encode(exp.as_bytes())),
//        })
//    }
//}
//

