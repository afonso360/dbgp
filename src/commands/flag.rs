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

macro_rules! simpl_flag_impl {
    ($type: ty) => {
        impl Flag for $type {
            fn format_flag(&self, flag: char) -> String {
                format!("-{} {}", flag, *self)
            }
        }

    }
}

simpl_flag_impl!(u8);
simpl_flag_impl!(u16);
simpl_flag_impl!(u32);
simpl_flag_impl!(u64);
simpl_flag_impl!(i8);
simpl_flag_impl!(i16);
simpl_flag_impl!(i32);
simpl_flag_impl!(i64);
simpl_flag_impl!(usize);
simpl_flag_impl!(isize);
simpl_flag_impl!(f32);
simpl_flag_impl!(f64);


impl<T> Flag for Option<T>
    where T: Flag {
    fn format_flag(&self, flag: char) -> String {
        match *self {
            Some(ref s) => s.format_flag(flag),
            None => "".to_string(),
        }
    }
}


impl Flag for AsRef<str> {
    fn format_flag(&self, flag: char) -> String {
        format!("-{} {}", flag, escape::escape(self.as_ref().to_owned()))
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


#[cfg(test)]
mod tests {
    use commands::flag::Flag;

    #[test]
    fn flag_base64_encode() {
        let buffer = [0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x21, 0x21];
        let result = "-d SGVsbG8gd29ybGQgISE=";
        assert_eq!(buffer.format_flag('d'), result);
    }
}
