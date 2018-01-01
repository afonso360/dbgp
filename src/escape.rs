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


#[cfg(test)]
mod tests {
    use escape::escape;

    #[test]
    fn escape_encloses_quotes_on_whitespace_space() {
        let command = r#"x['a b']"#;
        let result = r#""x['a b']""#;
        println!("command: {}\nresult: {}\nexpected: {}", command, escape(command.to_string()), result);
        assert_eq!(escape(command.to_string()), result);
    }

    #[test]
    fn escape_encloses_quotes_on_whitespace_tab() {
        let command = r#"x['a	b']"#;
        let result = r#""x['a	b']""#;
        assert_eq!(escape(command.to_string()), result);
    }

    #[test]
    fn escape_nothing() {
        let string = "$x['ab']#$:";
        assert_eq!(escape(string.to_string()), string);
    }

    #[test]
    fn escape_fuzz_1() {
        let string = "'RR٭'ű7";
        assert_eq!(escape(string.to_string()), string);
    }

    #[test]
    fn escape_fuzz_2() {
        let string = "R'٭R屴%'7";
        assert_eq!(escape(string.to_string()), string);
    }
}
