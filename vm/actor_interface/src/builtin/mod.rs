// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

pub mod account;
pub mod cron;
pub mod init;
pub mod market;
pub mod miner;
pub mod multisig;
pub mod power;
pub mod reward;
pub mod system;

use crate::ActorVersion;

use cid::Cid;

pub use fvm_shared::clock::EPOCH_DURATION_SECONDS;
pub use fvm_shared::smooth::FilterEstimate;
pub const EPOCHS_IN_DAY: clock::ChainEpoch = actorv0::EPOCHS_IN_DAY;

// Aliases for common addresses
pub static CHAOS_ACTOR_ADDR: &actorv0::CHAOS_ACTOR_ADDR = &actorv0::CHAOS_ACTOR_ADDR;
pub static BURNT_FUNDS_ACTOR_ADDR: &actorv0::BURNT_FUNDS_ACTOR_ADDR =
    &actorv0::BURNT_FUNDS_ACTOR_ADDR;
pub static RESERVE_ADDRESS: &actorv0::RESERVE_ADDRESS = &actorv0::RESERVE_ADDRESS;

/// Returns true if the code belongs to a builtin actor.
pub fn is_builtin_actor(code: &Cid) -> bool {
    actorv0::is_builtin_actor(code)
        || actorv2::is_builtin_actor(code)
        || actorv3::is_builtin_actor(code)
        || actorv4::is_builtin_actor(code)
        || actorv5::is_builtin_actor(code)
        || actorv6::is_builtin_actor(code)
}

/// Returns true if the code belongs to an account actor.
pub fn is_account_actor(code: &Cid) -> bool {
    actorv0::is_account_actor(code)
        || actorv2::is_account_actor(code)
        || actorv3::is_account_actor(code)
        || actorv4::is_account_actor(code)
        || actorv5::is_account_actor(code)
        || actorv6::is_account_actor(code)
}

/// Returns true if the code belongs to a singleton actor.
pub fn is_singleton_actor(code: &Cid) -> bool {
    actorv0::is_singleton_actor(code)
        || actorv2::is_singleton_actor(code)
        || actorv3::is_singleton_actor(code)
        || actorv4::is_singleton_actor(code)
        || actorv5::is_singleton_actor(code)
        || actorv6::is_singleton_actor(code)
}

/// Returns true if the code belongs to a miner actor.
pub fn is_miner_actor(code: &Cid) -> bool {
    code == &*actorv0::MINER_ACTOR_CODE_ID
        || code == &*actorv2::MINER_ACTOR_CODE_ID
        || code == &*actorv3::MINER_ACTOR_CODE_ID
        || code == &*actorv4::MINER_ACTOR_CODE_ID
        || code == &*actorv5::MINER_ACTOR_CODE_ID
        || code == &*actorv6::MINER_ACTOR_CODE_ID
}

/// Returns an actor's version or None if it was not a builtin
pub fn actor_version(code: &Cid) -> Option<ActorVersion> {
    if actorv6::is_builtin_actor(code) {
        Some(ActorVersion::V6)
    } else if actorv5::is_builtin_actor(code) {
        Some(ActorVersion::V5)
    } else if actorv4::is_builtin_actor(code) {
        Some(ActorVersion::V4)
    } else if actorv3::is_builtin_actor(code) {
        Some(ActorVersion::V3)
    } else if actorv2::is_builtin_actor(code) {
        Some(ActorVersion::V2)
    } else if actorv0::is_builtin_actor(code) {
        Some(ActorVersion::V0)
    } else {
        None
    }
}
