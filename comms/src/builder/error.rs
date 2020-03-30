// Copyright 2020, The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use crate::{connection_manager::ConnectionManagerError, peer_manager::PeerManagerError};
use derive_error::Error;

#[derive(Debug, Error)]
pub enum CommsBuilderError {
    PeerManagerError(PeerManagerError),
    ConnectionManagerError(ConnectionManagerError),
    /// Node identity not set. Call `with_node_identity(node_identity)` on [CommsBuilder]
    NodeIdentityNotSet,
    /// The PeerStorage was not provided to the CommsBuilder. Use `with_peer_storage` to set it.
    PeerStorageNotProvided,
    /// The messaging pipeline was not provided to the CommsBuilder. Use `with_messaging_pipeline` to set it.
    /// pipeline.
    MessagingPiplineNotProvided,
    /// Unable to receive a ConnectionManagerEvent within timeout
    ConnectionManagerEventStreamTimeout,
    /// ConnectionManagerEvent stream unexpectedly closed
    ConnectionManagerEventStreamClosed,
    /// Receiving on ConnectionManagerEvent stream lagged unexpectedly
    ConnectionManagerEventStreamLagged,
}
