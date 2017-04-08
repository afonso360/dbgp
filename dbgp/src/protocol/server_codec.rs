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

use tokio_proto::pipeline::ServerProto; //TODO: Change this to multiplexed
use tokio_io::{AsyncRead, AsyncWrite};
use std::{str, io};
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Framed, Decoder};


pub struct DbgpServerCodec;

impl Decoder for DbgpServerCodec {
    type Item = String; // TODO: change to XML
    type Error = io::Error;

    // The standard doesent make this very clear but the decode
    // should take a message in the following format
    // 10\0abcdefghij\0
    // length: 10
    // message: abcdefghij
    // so we need to parse the length as a string and into a number
    //TODO: Change '\n' to '\0'
    //TODO: Peek the length and then use len() on buf to check if the
    // message is fully here (make a benchmark for this first)
    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
        // look for the null character indicating the end of the length section
        if let Some(len_pos) = buf.iter().position(|&b| b == b'\n') {

            // Skip the length section and the null character and then
            // look for a null character indicating the end of hte packet
            if let Some(msg_pos) = buf.iter().skip(len_pos+1).position(|&b| b == b'\n') {
                // remove the serialized frame from the buffer.
                let length = buf.split_to(len_pos);

                // Also remove the '\0'
                buf.split_to(1);

                let message = buf.split_to(msg_pos);

                // Remove the '\0'
                buf.split_to(1);

                // Turn this data into a UTF string and return it in a Frame.
                match str::from_utf8(&message) {
                    Ok(s) => Ok(Some(s.to_string())),
                    Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                                 "invalid UTF-8")),
                }
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

impl Encoder for DbgpServerCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}


#[cfg(test)]
mod tests {
}
