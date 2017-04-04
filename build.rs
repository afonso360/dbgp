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

extern crate cheddar;

use std::env;
use std::io::{Read, Write};
use std::path::Path;
use std::fs::{OpenOptions, File};

static LICENSE_HEADER: &'static str = "\
/*
 * Copyright (c) the libdbgp contributors. All rights reserved.
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
";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir)
        .parent().unwrap() // there must be a better way to get the ouput dir
        .parent().unwrap()
        .parent().unwrap();

    let header = dest_path.clone()
        .join("include")
        .join("libdbgp.h");

    cheddar::Cheddar::new().expect("could not read manifest")
        .run_build(header.clone());



    let mut h_contents = String::new();
    {
        let mut tf = File::open(header.clone()).unwrap();
        tf.read_to_string(&mut h_contents).unwrap();
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(false)
        .truncate(false)
        .open(header)
        .unwrap();

    if let Err(e) = writeln!(file, "{}{}", LICENSE_HEADER, h_contents) {
        panic!("{}", e);
    }
}
