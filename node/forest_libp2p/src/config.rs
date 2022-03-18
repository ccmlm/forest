// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use libp2p::Multiaddr;
use networks::ChainConfig;
use serde::Deserialize;

/// Libp2p config for the Forest node.
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Libp2pConfig {
    /// Local address.
    pub listening_multiaddr: Multiaddr,
    /// Bootstrap peer list.
    pub bootstrap_peers: Vec<Multiaddr>,
    /// Mdns discovery enabled.
    pub mdns: bool,
    /// Kademlia discovery enabled.
    pub kademlia: bool,
    /// Target peer count.
    pub target_peer_count: u32,
}

impl Default for Libp2pConfig {
    fn default() -> Self {
        let chain_config = ChainConfig::default();
        let bootstrap_peers = chain_config
            .bootstrap_peers
            .iter()
            .map(|node| node.parse().unwrap())
            .collect();
        Self {
            listening_multiaddr: "/ip4/0.0.0.0/tcp/0".parse().unwrap(),
            bootstrap_peers,
            mdns: false,
            kademlia: true,
            target_peer_count: 75,
        }
    }
}
