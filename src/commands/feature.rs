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

use super::Command;
use super::flag::Flag;

command!("feature_set", struct FeatureSet {
    name: String: 'n',
    value: String: 'v'
});

command!("feature_get", struct FeatureGet {
    name: String: 'n'
});

#[cfg(test)]
mod tests {
    use super::{FeatureSet, FeatureGet};
    use commands::flag::Flag;
    use commands::Command;

    #[test]
    fn serialize_feature_set() {
        let result = "feature_set -i 0 -n namef -v valuef\0";
        assert_eq!((FeatureSet{
            name: "namef".into(),
            value: "valuef".into(),
        }).serialize(0), result);
    }

    #[test]
    fn serialize_feature_get() {
        let result = "feature_get -i 0 -n namef\0";
        assert_eq!((FeatureGet{
            name: "namef".into(),
        }).serialize(0), result);
    }
}
