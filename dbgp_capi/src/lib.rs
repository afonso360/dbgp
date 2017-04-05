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

// allows us to return strings to be freed with free()
#![feature(alloc_system)]
#![deny(missing_docs)]
//#![deny(warnings)]
#![doc(test(attr(allow(unused_variables), deny(warnings))))]

//! Implements the dbgp protocol

extern crate alloc_system;
extern crate dbgp;

use std::os::raw::c_char;
use std::ffi::CString;
use std::ffi::CStr;
use std::str;

/// This function escapes a string as defined in
/// [section 6](https://xdebug.org/docs-dbgp.php#id27)
/// of the dbgp protocol
///
/// The returning string must be deallocated by calling free()
/// Failure to allocate memory or a null pointer input will cause
/// a null pointer to be returned
/// Strings must be null terminated to be valid
//TODO: Fix the unwraps
//TODO: Check how many allocations we are doing with this operation
#[no_mangle]
pub extern "C"
fn dbgp_escape_string(input_str: *const c_char) -> *const c_char {
    if input_str.is_null() {
        return std::ptr::null();
    }

    let input = unsafe { CStr::from_ptr(input_str) };
    let input_buf: &str = str::from_utf8(input.to_bytes()).unwrap();
    let escaped_string = dbgp::escape::escape(input_buf.to_owned());
    return CString::new(escaped_string).unwrap().into_raw();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;

    #[test]
    fn escape_c_quotes() {
        let command = CString::new("\"$x[\"a b\"]\"").unwrap();
        let expected_result = "\"$x[\\\"a b\\\"]\"";

        let result_cstr = unsafe {
            CStr::from_ptr(dbgp_escape_string(command.as_ptr()))
        };

        assert_eq!(result_cstr.to_bytes(), expected_result.as_bytes());
    }

    #[test]
    fn escape_null_ptr() {
        assert_eq!(dbgp_escape_string(ptr::null()), ptr::null());
    }
}
