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

use base64;
use command::Command::*;
use escape;
use std::fmt;
use itertools::Itertools;

// TODO: Make command a trait
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
    ProxyInit { port: u32, ide_key: String, multi_debug: Option<bool> },
    ProxyStop { ide_key: String },
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
        data_page: Option<u32>,
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
        data_page: Option<u32>,

        property_address: Option<u32>,
        property_key: Option<u32>,
    },
    Source { begin_line: Option<u32>, end_line: Option<u32>, file_uri: String },
    StdOut { rediretion_type: RedirectionType },
    StdErr { rediretion_type: RedirectionType },


}

impl Command {
    pub fn get_name(&self) -> String {
        match *self {
            ProxyInit{..} => "proxyinit",
            ProxyStop{..} => "proxystop",
            Status => "status",
            FeatureGet{..} => "feature_get",
            FeatureSet{..} => "feature_set",
            Run => "run",
            StepInto => "step_into",
            StepOver => "step_over",
            StepOut => "step_out",
            Stop => "stop",
            Detach => "detach",
            BreakpointSet{..} => "breakpoint_set",
            BreakpointGet{..} => "breakpoint_get",
            BreakpointUpdate{..} => "breakpoint_update",
            BreakpointRemove{..} => "breakpoint_remove" ,
            BreapointList => "breapoint_list",
            StackDepth => "stack_depth",
            StackGet{..} => "stack_get" ,
            ContextNames{..} => "context_names" ,
            ContextGet{..} => "contextget",
            TypeMapGet => "typemap_get",
            PropertyGet{..} => "property_get",
            PropertySet{..} => "property_set",
            PropertyValue{..} => "property_value",
            Source{..} => "source",
            StdOut{..} => "stdout",
            StdErr{..} => "stderr",
            Break => "break",
        }.to_string()
    }

    pub fn build_command_string(&self, transaction_id: u32) -> String {
        let rest = match *self {
            TypeMapGet | Status |
                BreapointList | StackDepth | Run | StepInto |
                StepOver | StepOut | Stop | Detach => vec![],

            ProxyInit { port: p, ide_key: ref i, multi_debug: ref m } =>
                vec![p.format_flag('a'),
                i.format_flag('k'),
                m.format_flag('m')],

            ProxyStop { ide_key: ref i } => vec![i.format_flag('k')],
            FeatureGet{ name: ref n } => vec![n.format_flag('n')],
            FeatureSet{ name: ref n, value: ref v } =>
                vec![n.format_flag('n'), v.format_flag('v')],

            BreakpointSet{
                btype: ref bt,
                state: ref s,
                hit_value: ref hv,
                temporary: ref t
            } => vec![hv.format_flag('h'),
                t.format_flag('r'),
                s.format_flag('s'),
                bt.format_flag('t')],

            BreakpointRemove { breakpoint_id: bid } |
                BreakpointGet { breakpoint_id: bid } => vec![bid.format_flag('d')],

            BreakpointUpdate{
                breakpoint_id: bid,
                state: ref s,
                lineno: line,
                hit_value: hv,
                hit_condition: ref hc,
            } => vec![bid.format_flag('d'),
                s.format_flag('s'),
                line.format_flag('n'),
                hv.format_flag('h'),
                hc.format_flag('o')],

            StackGet { stack_depth: sd } |
                ContextNames { stack_depth: sd } => vec![sd.format_flag('d')],

            ContextGet{ stack_depth: sd, context_id: ci } =>
                vec![sd.format_flag('d'), ci.format_flag('c')],

            PropertyGet {
                stack_depth: sd,
                context_id: ci,
                property_long_name: ref pln,
                max_data: md,
                data_page: ref dp,
                property_key: pk,
            } => vec![pln.format_flag('n'),
                sd.format_flag('d'),
                ci.format_flag('c'),
                md.format_flag('m'),
                dp.format_flag('p'),
                pk.format_flag('k')],

            PropertySet {
                stack_depth: sd,
                context_id: ci,
                property_long_name: ref pln,
                max_data: md,
                data_type: ref dt,
                property_address: pa,
            } => vec![ pln.format_flag('n'),
                sd.format_flag('d'),
                ci.format_flag('c'),
                md.format_flag('m'),
                dt.format_flag('t'),
                pa.format_flag('a')],

            PropertyValue {
                stack_depth: sd,
                context_id: ci,
                property_long_name: ref pln,
                max_data: md,
                data_page: ref dp,
                property_address: pa,
                property_key: pk,
            } => vec![ pln.format_flag('n'),
                sd.format_flag('d'),
                ci.format_flag('c'),
                md.format_flag('m'),
                dp.format_flag('p'),
                pa.format_flag('a'),
                pk.format_flag('k')],

            Source { begin_line: bl, end_line: el, file_uri: ref fu, } =>
                vec![bl.format_flag('f'), el.format_flag('n'), fu.format_flag('s')],

            StdOut { rediretion_type: ref rt } |
                StdErr { rediretion_type: ref rt } => vec![rt.format_flag('c')],
        }.iter()
         .join(" ");

        format!("{} -i {} {}", self.get_name(), transaction_id, rest)
    }
}

#[cfg(test)]
mod tests {
    use command::Command;

    #[test]
    fn test_command() {
        let c = Command::ProxyInit {
            port: 9100,
            ide_key: "ied_caslc".to_string(),
            multi_debug: Some(true),
        };
        assert_eq!(c.build_command_string(10),
            "proxyinit -i 10 -a 9100 -k ied_caslc -m 1".to_string());
    }
}
