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
    StdIn {
        redirect: bool,
        data: Option<String>,
    },
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
    SpawnpointUpdate { lineno: Option<u32>, state: Option<SpawnpointState> },
    SpawnpointRemove { id: u32 },
    SpawnpointList,

    Interact { mode: u32 },

}

impl Command {
    pub fn get_name(&self) -> String {
        match *self {
            ProxyInit{..} => "proxyinit".to_string(),
            ProxyStop{..} => "proxystop".to_string(),
            Status => "status".to_string(),
            FeatureGet{..} => "feature_get".to_string(),
            FeatureSet{..} => "feature_set".to_string(),
            Run => "run".to_string(),
            StepInto => "step_into".to_string(),
            StepOver => "step_over".to_string(),
            StepOut => "step_out".to_string(),
            Stop => "stop".to_string(),
            Detach => "detach".to_string(),
            BreakpointSet{..} => "breakpoint_set".to_string(),
            BreakpointGet{..} => "breakpoint_get".to_string(),
            BreakpointUpdate{..} => "breakpoint_update".to_string(),
            BreakpointRemove{..} => "breakpoint_remove".to_string() ,
            BreapointList => "breapoint_list".to_string(),
            StackDepth => "stack_depth".to_string(),
            StackGet{..} => "stack_get".to_string() ,
            ContextNames{..} => "context_names".to_string() ,
            ContextGet{..} => "contextget".to_string(),
            TypeMapGet => "typemap_get".to_string(),
            PropertyGet{..} => "property_get".to_string(),
            PropertySet{..} => "property_set".to_string(),
            PropertyValue{..} => "property_value".to_string(),
            Source{..} => "source".to_string(),
            StdOut{..} => "stdout".to_string(),
            StdErr{..} => "stderr".to_string(),
            StdIn{..} => "stdin".to_string(),
            Break => "break".to_string(),
            Eval{..} => "eval".to_string(),
            Expr => "expr".to_string(),
            Exec => "exec".to_string(),
            SpawnpointSet{..} => "spawnpoint_set".to_string(),
            SpawnpointGet{..} => "spawnpoint_get".to_string(),
            SpawnpointUpdate{..} => "spawnpoint_update".to_string(),
            SpawnpointRemove{..} => "spawnpoint_remove".to_string(),
            SpawnpointList => "spawnpoint_list".to_string(),
            Interact{..} => "interact".to_string(),
        }
    }

    pub fn build_command_string(&self, transaction_id: u32) -> String {
        format!("{} -i {} {}", self.get_name(), transaction_id,
        match *self {
            TypeMapGet | Break | Status | SpawnpointList |
                BreapointList | StackDepth | Run | StepInto |
                StepOver | StepOut | Stop | Detach => "".to_string(),

            ProxyInit { port: p, ide_key: ref i, multi_debug: ref m } =>
                format!("{} {} {}",
                        p.format_flag('a'),
                        i.format_flag('k'),
                        m.format_flag('m')),

            ProxyStop { ide_key: ref i } => i.format_flag('k'),
            FeatureGet{ name: ref n } => n.format_flag('n'),
            FeatureSet{ name: ref n, value: ref v } =>
                format!("{} {}",
                        n.format_flag('n'),
                        v.format_flag('v')),

            BreakpointSet{ btype: ref bt, state: ref s, hit_value: ref hv, temporary: ref t } =>
                format!("{} {} {} {}",
                        hv.format_flag('h'),
                        t.format_flag('r'),
                        s.format_flag('s'),
                        bt.format_flag('t')),

            BreakpointRemove { breakpoint_id: bid } |
                BreakpointGet { breakpoint_id: bid } => bid.format_flag('d'),

            BreakpointUpdate{
                breakpoint_id: bid,
                state: ref s,
                lineno: line,
                hit_value: hv,
                hit_condition: ref hc,
            } => format!("{} {} {} {} {}",
                         bid.format_flag('d'),
                         s.format_flag('s'),
                         line.format_flag('n'),
                         hv.format_flag('h'),
                         hc.format_flag('o')),
            StackGet { stack_depth: sd } |
                ContextNames { stack_depth: sd } => sd.format_flag('d'),

            ContextGet{ stack_depth: sd, context_id: ci } =>
                format!("{} {}", sd.format_flag('d'), ci.format_flag('c')),

            PropertyGet {
                stack_depth: sd,
                context_id: ci,
                property_long_name: ref pln,
                max_data: md,
                data_page: ref dp,
                property_key: pk,
            } => format!("{} {} {} {} {} {}",
                         pln.format_flag('n'),
                         sd.format_flag('d'),
                         ci.format_flag('c'),
                         md.format_flag('m'),
                         dp.format_flag('p'),
                         pk.format_flag('k')),

            PropertySet {
                stack_depth: sd,
                context_id: ci,
                property_long_name: ref pln,
                max_data: md,
                data_type: ref dt,
                property_address: pa,
            } => format!("{} {} {} {} {} {}",
                         pln.format_flag('n'),
                         sd.format_flag('d'),
                         ci.format_flag('c'),
                         md.format_flag('m'),
                         dt.format_flag('t'),
                         pa.format_flag('a')),

            PropertyValue {
                stack_depth: sd,
                context_id: ci,
                property_long_name: ref pln,
                max_data: md,
                data_page: ref dp,
                property_address: pa,
                property_key: pk,
            } => format!("{} {} {} {} {} {} {}",
                         pln.format_flag('n'),
                         sd.format_flag('d'),
                         ci.format_flag('c'),
                         md.format_flag('m'),
                         dp.format_flag('p'),
                         pa.format_flag('a'),
                         pk.format_flag('k')),

            Source { begin_line: bl, end_line: el, file_uri: ref fu, } =>
                format!("{} {} {}",
                         bl.format_flag('f'),
                         el.format_flag('n'),
                         fu.format_flag('s')),

            StdOut { rediretion_type: ref rt } => rt.format_flag('c'),
            StdErr { rediretion_type: ref rt } => rt.format_flag('c'),
            StdIn { redirect: r, data: ref data } =>
                format!("{} {}", r.format_flag('c'), match *data {
                    Some(ref s) => format!("-- {}", base64::encode(s.as_bytes())),
                    None => "".to_string(),
                }),
            Eval { stack_depth: sd, data_page: ref dp } =>
                format!("{} {}", sd.format_flag('d'), dp.format_flag('p')),

            Expr => "".to_string(),
            Exec => "".to_string(),

            SpawnpointSet {
                filename: ref file,
                lineno: line,
                state: ref state,
            } => format!("{} {} {}",
                         file.format_flag('f'),
                         line.format_flag('n'),
                         state.format_flag('s')),

            SpawnpointRemove { id: i } | SpawnpointGet { id: i } |
                SpawnpointRemove { id: i } => i.format_flag('i'),

            SpawnpointUpdate { lineno: line, state: ref state } =>
                format!("{} {}", line.format_flag('n'), state.format_flag('s')),
            Interact { mode: m } => m.format_flag('m'),
        })
    }
}

trait ToFlag {
    fn format_flag(&self, flag: char) -> String;
}

// TODO: Refactor this once the bug has been fixed
// We could use Specialization here but rust has a bug
// https://github.com/rust-lang/rust/issues/41140

impl ToFlag for u32 {
    fn format_flag(&self, flag: char) -> String {
        format!("-{} {}", flag, *self)
    }
}

impl<T> ToFlag for Option<T>
    where T: ToFlag {
    fn format_flag(&self, flag: char) -> String {
        match *self {
            Some(ref s) => s.format_flag(flag),
            None => "".to_string(),
        }
    }
}


impl ToFlag for String {
    fn format_flag(&self, flag: char) -> String {
        format!("-{} {}", flag, escape::escape(self.clone()))
    }
}


impl ToFlag for bool {
    fn format_flag(&self, flag: char) -> String {
        match *self {
            true => format!("-{} 1", flag),
            false => format!("-{} 0", flag),
        }
    }
}

impl ToFlag for SpawnpointState {
    fn format_flag(&self, flag: char) -> String {
        match *self {
            SpawnpointState::Enabled => format!("-{} enabled", flag),
            SpawnpointState::Disabled => format!("-{} disabled", flag),
        }
    }
}

impl ToFlag for RedirectionType {
    fn format_flag(&self, flag: char) -> String {
        match *self {
            RedirectionType::Disable => format!("-{} 0", flag),
            RedirectionType::Copy => format!("-{} 1", flag),
            RedirectionType::Redirect => format!("-{} 2", flag),
        }
    }
}

impl ToFlag for BreakpointState {
    fn format_flag(&self, flag: char) -> String {
        match *self {
            BreakpointState::Enabled => format!("-{} enabled", flag),
            BreakpointState::Disabled => format!("-{} disabled", flag),
        }
    }
}


impl ToFlag for BreakpointType {
    fn format_flag(&self, flag: char) -> String {
        format!("-{} {}", flag, match *self {
            BreakpointType::Line{ filename: ref f, lineno: ref l } =>
                format!("line -f {} -l {}", f, l),
            BreakpointType::Call{ function: ref m } =>
                format!("call -m {}", m),
            BreakpointType::Return{ function: ref m } =>
                format!("return -m {}", m),
            BreakpointType::Exception{ exception: ref x } =>
                format!("exception -x {}", x),
            BreakpointType::Conditional{ expression: ref exp, filename: ref f } =>
                format!("conditional -f {} -- {}", f, base64::encode(exp.as_bytes())),
            BreakpointType::Watch{ expression: ref exp } =>
                format!("watch -- {}", base64::encode(exp.as_bytes())),
        })
    }
}
