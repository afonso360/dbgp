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

// TODO: Can we merge spawnpoints with breakpoints?

pub enum BreakpointType {
    Line{ filename: String, lineno: u32 },
    Call{ function: String },
    Return{ function: String },
    Exception{ exception: String },
    Conditional{ expression: String, filename: String },
    Watch{ expression: String },
}

pub enum BreakpointState {
    Enabled,
    Disabled
}

pub enum SpawnpointState {
    Enabled,
    Disabled
}

pub enum RedirectionType {
    /// stdout/stderr output goes to regular place, but not to Debug Client
    Disable,
    /// stdout/stderr output goes to both regular destination and Debug Client
    Copy,
    /// stdout/stderr output goes to Debug Client only.
    Redirect
}

// Change all files to URI? or is there a filename and file URI
pub enum Command {
    Status,
    FeatureGet{ name: String },
    FeatureSet{ name: String, value: String },
    Run,
    StepInto,
    StepOver,
    StepOut,
    Stop,
    Detach,
    BreakpointSet{
        btype: BreakpointType,
        state: Option<BreakpointState>,
        /// used with the hit condition to determine if should
        /// break; a value of zero indicates hit count processing
        /// is disabled for this breakpoint
        /// [optional, defaults to zero (i.e. disabled)]
        hit_value: u32,

        //Translates to 1 or 0 when converting to string
        temporary: bool,
    },
    BreakpointGet{ breakpoint_id: u32 },
    BreakpointUpdate{
        breakpoint_id: u32,
        // At least one of these should be in here before we send
        state: Option<BreakpointState>,
        lineno: Option<u32>,
        hit_value: Option<u32>,
        hit_condition: Option<String>,
    },
    BreakpointRemove { breakpoint_id: u32 },
    BreapointList,
    StackDepth,
    StackGet { stack_depth: Option<u32> },
    ContextNames { stack_depth: Option<u32> },
    ContextGet{ stack_depth: Option<u32>, context_id: Option<u32> },
    TypeMapGet,
    PropertyGet {
        stack_depth: Option<u32>,
        context_id: Option<u32>,
        property_long_name: String,
        /// Max data size to retireve
        max_data: Option<u32>,

        /// Optional for arras, hashes objects, etc
        //TODO: is this a string?
        data_page: Option<String>,
        property_key: Option<u32>,
    },
    PropertySet {
        stack_depth: Option<u32>,
        context_id: Option<u32>,
        property_long_name: String,
        /// Max data size to retireve
        max_data: Option<u32>,
        data_type: Option<String>,

        //TODO: do we need data_page here?

        property_address: Option<u32>,
    },
    PropertyValue {
        stack_depth: Option<u32>,
        context_id: Option<u32>,
        property_long_name: String,
        /// Max data size to retireve
        max_data: Option<u32>,

        /// Optional for arras, hashes objects, etc
        //TODO: is this a string?
        data_page: Option<String>,

        property_address: Option<u32>,
        property_key: Option<u32>,
    },
    Source { begin_line: Option<u32>, end_line: Option<u32>, file_uri: String },
    StdOut { rediretion_type: RedirectionType },
    StdErr { rediretion_type: RedirectionType },

    //Extended commands
    StdIn { redirect: bool },
    Break,
    Eval {
        stack_depth: Option<u32>,

        /// Optional for arras, hashes objects, etc
        //TODO: is this a string?
        data_page: Option<String>,
    },
    //TODO: Check this
    Expr,
    Exec,

    SpawnpointSet {
        filename: Option<String>,
        lineno: Option<u32>,
        state: Option<SpawnpointState>,
    },
    SpawnpointGet { id: u32 },
    SpawnpointUpdate {
        lineno: Option<u32>,
        state: Option<SpawnpointState>,
    },
    SpawnpointRemove { id: u32 },
    SpawnpointList,

    Interact { mode: u32 },
}
