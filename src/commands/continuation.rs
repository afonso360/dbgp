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

command!("run", struct Run {});

command!("step_into", struct StepInto {});

command!("step_over", struct StepOver {});

command!("step_out", struct StepOut {});

command!("stop", struct Stop {});

command!("detach", struct Detach {});

#[cfg(test)]
mod tests {
    #[test]
    fn serialize_run() {
        command_serialize_test!(Run{}, 0, "run -i 0\0")
    }

    #[test]
    fn serialize_step_into() {
        command_serialize_test!(StepInto{}, 1, "step_into -i 1\0")
    }

    #[test]
    fn serialize_step_over() {
        command_serialize_test!(StepOver{}, 2, "step_over -i 2\0")
    }

    #[test]
    fn serialize_step_out() {
        command_serialize_test!(StepOut{}, 3, "step_out -i 3\0")
    }

    #[test]
    fn serialize_stop() {
        command_serialize_test!(Stop{}, 4, "stop -i 4\0")
    }

    #[test]
    fn serialize_detach() {
        command_serialize_test!(Detach{}, 5, "detach -i 5\0")
    }
}
