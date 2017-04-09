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

pub mod client_codec;
//pub mod server_codec;

//pub use protocol::server_codec::ServerCodec;
pub use protocol::client_codec::ClientCodec;
use tokio_proto::pipeline::ServerProto; //TODO: Change this to multiplexed
use tokio_io::{AsyncRead, AsyncWrite};
use std::{str, io};
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Framed, Decoder};

pub struct DbgpProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for DbgpProto {
    type Request = String;
    type Response = String;
    type Transport = Framed<T, ClientCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(ClientCodec))
    }
}

