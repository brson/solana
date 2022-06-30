#![allow(incomplete_features)]
#![cfg_attr(RUSTC_WITH_SPECIALIZATION, feature(specialization))]
#![cfg_attr(RUSTC_NEEDS_PROC_MACRO_HYGIENE, feature(proc_macro_hygiene))]

// Allows macro expansion of `use ::solana_sdk::*` to work within this crate
extern crate self as solana_sdk;

#[cfg(feature = "full")]
pub use signer::signers;
pub use solana_program::*;

pub mod account;
pub mod account_utils;
pub mod builtins;
pub mod client;
pub mod commitment_config;
pub mod compute_budget;
pub mod derivation_path;
pub mod deserialize_utils;
pub mod ed25519_instruction;
#[doc(hidden)] // hack - fix duplicate docs #26211
pub mod entrypoint;
#[doc(hidden)] // hack - fix duplicate docs #26211
pub mod entrypoint_deprecated;
pub mod epoch_info;
pub mod example_mocks;
pub mod exit;
#[doc(hidden)] // hack - fix duplicate docs #26211
pub mod feature;
pub mod feature_set;
pub mod fee;
pub mod genesis_config;
pub mod hard_forks;
#[doc(hidden)] // hack - fix duplicate docs #26211
pub mod hash;
pub mod inflation;
pub mod keyed_account;
pub mod log;
pub mod native_loader;
pub mod nonce_account;
pub mod packet;
pub mod poh_config;
pub mod precompiles;
#[doc(hidden)] // hack - fix duplicate docs #26211
pub mod program_utils;
#[doc(hidden)] // hack - fix duplicate docs #26211
pub mod pubkey;
pub mod quic;
pub mod recent_blockhashes_account;
pub mod rpc_port;
pub mod secp256k1_instruction;
pub mod shred_version;
pub mod signature;
pub mod signer;
pub mod system_transaction;
pub mod timing;
pub mod transaction;
pub mod transaction_context;
pub mod transport;
pub mod wasm;

pub use solana_sdk_macro::pubkeys;
#[rustversion::since(1.46.0)]
pub use solana_sdk_macro::respan;

// Unused `solana_sdk::program_stubs!()` macro retained for source backwards compatibility with older programs
#[macro_export]
#[deprecated(
    since = "1.4.3",
    note = "program_stubs macro is obsolete and can be safely removed"
)]
macro_rules! program_stubs {
    () => {};
}

/// Convenience macro for `AddAssign` with saturating arithmetic.
/// Replace by `std::num::Saturating` once stable
#[macro_export]
macro_rules! saturating_add_assign {
    ($i:expr, $v:expr) => {{
        $i = $i.saturating_add($v)
    }};
}

#[macro_use]
extern crate serde_derive;
pub extern crate bs58;
extern crate log as logger;

#[macro_use]
extern crate solana_frozen_abi_macro;

#[cfg(test)]
mod tests {
    #[test]
    fn test_saturating_add_assign() {
        let mut i = 0u64;
        let v = 1;
        saturating_add_assign!(i, v);
        assert_eq!(i, 1);

        i = u64::MAX;
        saturating_add_assign!(i, v);
        assert_eq!(i, u64::MAX);
    }
}
