//! Network clients of Solana nodes.
//!
//! A Solana cluster is composed of Solana nodes, some of which participate as
//! validators. Other programs interact with the network as clients of those
//! nodes.
//!
//! The main types of clients are _RPC clients_, and _pubsub clients_, which
//! query, transact with, and receive events from Solana nodes via the [Solana
//! JSON RPC API][JSON-RPC].
//!
//! [JSON-RPC]: https://docs.solana.com/developing/clients/jsonrpc-api
//!
//! Clients can be either blocking or nonblocking, in which case they are
//! written in asyncronous Rust, on top of the [tokio] runtime. The nonblocking
//! clients are generally located under the [`nonblocking`] module.
//!
//! [tokio]: https://docs.rs/tokio
//!
//! ## Solana client types
//!
//! **RPC clients** are for sending queries and transactions to a Solana node.
//! The blocking type is [`rpc_client::RpcClient`]. The nonblocking type is
//! [`nonblocking::rpc_client::RpcClient`].
//!
//! **Pubsub clients** are for receiving event notifications from a Solana node,
//! and can be more efficient than querying with an RPC client. The blocking
//! type is [`pubsub_client::PubsubClient`]. The nonblocking type is
//! [`nonblocking::pubsub_client::PubsubClient`].
//!
//! **TPU clients** communicate directly with the [TPU] of the [slot leader] to
//! fill blocks with transactions as fast as possible. Validators are TPU
//! clients of each other. Most users do not need to use TPU clients, though
//! tools that need to submit many transactions may want to communicate directly
//! with the TPU. An example of a tool that communicates directly with the TPU
//! is `solana program deploy`, which posts many transactions to deploy a single
//! program. Of the multiple TPU clients the [`quic_client`] is recommended.
//!
//! [TPU]: https://docs.solana.com/validator/tpu
//! [slot leader]: https://docs.solana.com/cluster/leader-rotation
//!
//! ## Client crates
//!
//! For historical reasons this `solana-client` crate is a façade that reexports
//! every client type from other crates. Instead of using this crate
//! users may want to use crates specific to their purpose:
//!
//! - [`solana-rpc-client`] - The RPC client.
//! - [`solana-pubsub-client`] - The Pubsub client.
//! - [`solana-quic-client`] - The QUIC TPU client.
//! - [`solana-udp-client`] - The UDP TPU client.
//! - [`solana-tpu-client`] - The legacy TPU client.
//!
//! [`solana-rpc-client`]: https://docs.rs/solana-rpc-client
//! [`solana-pubsub-client`]: https://docs.rs/solana-pubsub-client
//! [`solana-quic-client`]: https://docs.rs/solana-quic-client
//! [`solana-udp-client`]: https://docs.rs/solana-udp-client
//! [`solana-tpu-client`]: https://docs.rs/solana-tpu-client

#![allow(clippy::integer_arithmetic)]

pub mod connection_cache;
pub mod nonblocking;
pub mod quic_client;

/// A historical artifact used for internal testing.
pub mod thin_client;
pub mod tpu_client;
pub mod tpu_connection;
pub mod transaction_executor;
pub mod udp_client;

#[macro_use]
extern crate solana_metrics;

pub use solana_rpc_client::mock_sender_for_cli;

pub mod blockhash_query {
    pub use solana_rpc_client_nonce_utils::blockhash_query::*;
}
pub mod client_error {
    pub use solana_rpc_client_api::client_error::{
        reqwest, Error as ClientError, ErrorKind as ClientErrorKind, Result,
    };
}
/// Durable transaction nonce helpers.
pub mod nonce_utils {
    pub use solana_rpc_client_nonce_utils::*;
}
pub mod pubsub_client {
    pub use solana_pubsub_client::pubsub_client::*;
}
/// Communication with a Solana node over RPC.
///
/// Software that interacts with the Solana blockchain, whether querying its
/// state or submitting transactions, communicates with a Solana node over
/// [JSON-RPC], using the [`RpcClient`] type.
///
/// [JSON-RPC]: https://www.jsonrpc.org/specification
/// [`RpcClient`]: crate::rpc_client::RpcClient
pub mod rpc_client {
    pub use solana_rpc_client::rpc_client::*;
}
pub mod rpc_config {
    pub use solana_rpc_client_api::config::*;
}
/// Implementation defined RPC server errors
pub mod rpc_custom_error {
    pub use solana_rpc_client_api::custom_error::*;
}
pub mod rpc_deprecated_config {
    pub use solana_rpc_client_api::deprecated_config::*;
}
pub mod rpc_filter {
    pub use solana_rpc_client_api::filter::*;
}
pub mod rpc_request {
    pub use solana_rpc_client_api::request::*;
}
pub mod rpc_response {
    pub use solana_rpc_client_api::response::*;
}
/// A transport for RPC calls.
pub mod rpc_sender {
    pub use solana_rpc_client::rpc_sender::*;
}
