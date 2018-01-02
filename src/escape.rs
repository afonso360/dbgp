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

//! Implements escapes according to section 6

/// This function escapes a string as defined in
/// [section 6](https://xdebug.org/docs-dbgp.php#id27)
/// of the dbgp protocol
///
/// Currently the only defined escape is to escape inner quotes
pub fn escape<S: Into<String>>(string: S) -> String {
    let mut string = string.into();
    if !needs_escape(&string) {
        return string;
    }

    string = string.replace('"', r#"\""#);

    format!(r#""{}""#, string)
}

pub fn needs_escape<SR: AsRef<str>>(string: SR) -> bool {
    let string = string.as_ref();

    // Technically we should only escape if we find a Space, but i've used
    // is_whitespace because I think some clients wouldn't handle tabs correctly
    !( string.find(char::is_whitespace) == None && string.find('"') == None )
}

pub fn unescape<S: Into<String>>(string: S) -> String {
    let mut string = string.into();
    if !string.starts_with('"') && !string.ends_with('"') {
        return string;
    }

    string = string.replace(r#"\""#, "\"");
    string.chars()
        .skip(1)
        .take(string.chars().count() - 2)
        .collect()
}



#[cfg(test)]
mod tests {
    use escape::{unescape, escape};

    macro_rules! roundtrip_test {
        ($orig: expr, $new: expr) => {
            let escaped = escape($orig);
            assert_eq!(escaped, $new);

            let unescaped = unescape(escaped);
            assert_eq!(unescaped, $orig);
        }
    }

    #[test]
    fn escape_encloses_quotes_on_whitespace_space() {
        roundtrip_test!(r#"x['a b']"#, r#""x['a b']""#);
    }

    #[test]
    fn escape_encloses_quotes_on_whitespace_tab() {
        roundtrip_test!(r#"x['a	b']"#, r#""x['a	b']""#);
    }

    #[test]
    fn escape_nothing() {
        let string = "$x['ab']#$:";
        roundtrip_test!(string, string);
    }

    #[test]
    fn fuzz_1() {
        let string = "'RR٭'ű7";
        roundtrip_test!(string, string);
    }

    #[test]
    fn fuzz_2() {
        let string = "R'٭R屴%'7";
        roundtrip_test!(string, string);
    }

    #[test]
    #[ignore]
    fn fuzz_3() {
        let string = r#"'R"\"#;
        roundtrip_test!(string, string);
    }


    quickcheck! {
        fn escape_roundtrip(test: String) -> bool {
            let original = test.clone();
            original == unescape(escape(test))
        }

        #[ignore]
        fn multi_escape_roundtrip(test: String) -> bool {
            let size = 10;
            let original = test.clone();
            let mut result = test;

            for _ in 0..size {
                result = escape(result);
            }

            for _ in 0..size {
                result = unescape(result);
            }

            original == result
        }
    }
}
