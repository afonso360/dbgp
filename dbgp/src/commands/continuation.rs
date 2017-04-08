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

use super::{Command, Response};
use super::flag::Flag;
use xml::reader::XmlEvent;

response!(struct RunResponse {});

command!("run",
         struct Run {},
         RunResponse,
         |i: &Run, xml: XmlEvent| {
    RunResponse{}
});

response!(struct StepIntoResponse {});

command!("step_into",
         struct StepInto {},
         StepIntoResponse,
         |i: &StepInto, xml: XmlEvent| {
    StepIntoResponse{}
});

response!(struct StepOverResponse {});

command!("step_over",
         struct StepOver {},
         StepOverResponse,
         |i: &StepOver, xml: XmlEvent| {
    StepOverResponse{}
});

response!(struct StepOutResponse {});

command!("step_out",
         struct StepOut {},
         StepOutResponse,
         |i: &StepOut, xml: XmlEvent| {
    StepOutResponse{}
});

response!(struct StopResponse {});

command!("stop",
         struct Stop {},
         StopResponse,
         |i: &Stop, xml: XmlEvent| {
    StopResponse{}
});

response!(struct DetachResponse {});

command!("detach",
         struct Detach {},
         DetachResponse,
         |i: &Detach, xml: XmlEvent| {
    DetachResponse{}
});
