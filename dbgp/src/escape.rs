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
// TODO: Consider having a Into<String> as an argument
// TODO: This is a mess
// TODO: REDO THIS PLEASE
pub fn escape(string: String) -> String {
    let escape_chars = vec!['"', '\''];
    let mut inside_quotes = ' ';
    let mut end = String::new();
    let mut index: usize = 0;
    let mut last_quote_index: usize = 0;

    for c in string.chars() {
        if escape_chars.contains(&c) && inside_quotes != c {
            inside_quotes = c;
        } else if escape_chars.contains(&c) && inside_quotes == c {
            end.push('\\');
            last_quote_index = index;
            index += 1;
        }

        end.push(c);
        index += 1;
    }

    if last_quote_index != 0 {
        end.remove(last_quote_index);
    }
    end
}


#[cfg(test)]
mod tests {
    use escape::escape;

    #[test]
    fn escape_single_quotes() {
        let command = "'$x['a b']'";
        let result = "'$x[\\'a b\\']'";
        assert_eq!(escape(command.to_string()), result);
    }

    #[test]
    fn escape_single_quotes_unchanged() {
        let command = "'$x[\"a b\"]'";
        let result = "'$x[\"a b\"]'";
        assert_eq!(escape(command.to_string()), result);
    }

    #[test]
    fn escape_quotes() {
        let command = "\"$x[\"a b\"]\"";
        let result = "\"$x[\\\"a b\\\"]\"";
        assert_eq!(escape(command.to_string()), result);
    }

    #[test]
    fn escape_inner_single_quotes() {
        let command = "\"$x['a b']\"";
        let result = "\"$x['a b']\"";
        assert_eq!(escape(command.to_string()), result);
    }

    #[test]
    fn escape_nothing() {
        let string = "$x['a b']#$:";
        assert_eq!(escape(string.to_string()), string);
    }
}
