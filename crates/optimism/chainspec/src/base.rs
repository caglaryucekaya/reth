//! Chain specification for the Base Mainnet network.

use alloc::{sync::Arc, vec};

use alloy_chains::Chain;
use alloy_primitives::{b256, U256};
use reth_chainspec::{once_cell_set, BaseFeeParams, BaseFeeParamsKind, ChainSpec};
use reth_ethereum_forks::{EthereumHardfork, Hardfork};
use reth_optimism_forks::OpHardfork;

use crate::{LazyLock, OpChainSpec};

/// The Base mainnet spec
pub static BASE_MAINNET: LazyLock<Arc<OpChainSpec>> = LazyLock::new(|| {
    OpChainSpec {
        inner: ChainSpec {
            chain: Chain::base_mainnet(),
            genesis: serde_json::from_str(include_str!("../res/genesis/base.json"))
                .expect("Can't deserialize Base genesis json"),
            genesis_hash: once_cell_set(b256!(
                "f712aa9241cc24369b143cf6dce85f0902a9731e70d66818a3a5845b296c73dd"
            )),
            paris_block_and_final_difficulty: Some((0, U256::from(0))),
            hardforks: OpHardfork::base_mainnet(),
            base_fee_params: BaseFeeParamsKind::Variable(
                vec![
                    (EthereumHardfork::London.boxed(), BaseFeeParams::optimism()),
                    (OpHardfork::Canyon.boxed(), BaseFeeParams::optimism_canyon()),
                ]
                .into(),
            ),
            ..Default::default()
        },
    }
    .into()
});
