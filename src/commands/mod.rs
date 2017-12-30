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


// TODO: Runtime feature detection?
// TODO: Tests for this macro
// TODO: Give an example of this macro

/// To use this macro member types must implement the Flag trait
///
/// Flags will be passed in the same order as member elements of
/// this struct, this means that the base64 data of some commands
/// has to be the last member of the struct
macro_rules! command {
    ($dbgp_name: expr,
     struct $name:ident {
        $($fname:ident: $ftype:ty: $flag: expr),*
     }) => {
        #[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
        pub struct $name {
            $($fname : $ftype),*
        }

        impl ::commands::Command for $name {
            fn serialize(&self, transaction_id: u32) -> String {
                use commands::flag::Flag;

                format!("{}\0", [
                    $dbgp_name.to_string(),
                    format!("-i {}", transaction_id),
                    $(self.$fname.format_flag($flag)),*
                ].join(" "))
            }
        }
    }
}

pub mod flag;
pub mod feature;
pub mod status;

pub use self::flag::Flag;
pub use self::feature::{FeatureGet, FeatureSet};
pub use self::status::Status;

//mod base;
//mod eval;
//mod spawnpoints;
//mod rbreak;
//mod interact;
//mod notifications;
//mod stdin;
//mod proxy;
//mod continuation;
//mod io;
//mod source;
//mod stack;
//mod context;
//mod typemap;
//mod property;
//mod breakpoint;
//mod init;


pub trait Command {
    /// Outputs a DGBP compatible command string
    fn serialize(&self, transaction_id: u32) -> String;
}
