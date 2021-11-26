
//! Autogenerated weights for `pallet_ip_staking`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-07, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_staking.
pub trait WeightInfo {
    fn register() -> Weight;
    /// n - number of existing stakers on the contract
    fn unregister(n: u32) -> Weight;
    fn enable_developer_pre_approval() -> Weight;
    fn developer_pre_approval() -> Weight;
    fn bond_and_stake() -> Weight;
    fn unbond_unstake_and_withdraw() -> Weight;
    /// n - total number of payees
    fn claim(n: u32) -> Weight;
    fn force_new_era() -> Weight;
}

/// Weights for pallet_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: IpStaking RegisteredDevelopers (r:1 w:1)
	// Storage: IpStaking RegisteredIp (r:1 w:1)
	// Storage: IpStaking PreApprovalIsEnabled (r:1 w:0)
	fn register() -> Weight {
		(63_974_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: IpStaking RegisteredDevelopers (r:1 w:1)
	// Storage: IpStaking CurrentEra (r:1 w:0)
	// Storage: IpStaking ContractEraStake (r:3 w:0)
	// Storage: IpStaking EraRewardsAndStakes (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: IpStaking Ledger (r:25 w:25)
	// Storage: Balances Locks (r:25 w:25)
	fn unregister(n: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 68_000
			.saturating_add((43_004_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(n as Weight)))
	}
	// Storage: IpStaking PreApprovalIsEnabled (r:0 w:1)
	fn enable_developer_pre_approval() -> Weight {
		(3_132_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: IpStaking PreApprovedDevelopers (r:1 w:1)
	fn developer_pre_approval() -> Weight {
		(9_337_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: IpStaking RegisteredIp (r:1 w:0)
	// Storage: IpStaking RegisteredDevelopers (r:1 w:0)
	// Storage: IpStaking Ledger (r:1 w:1)
	// Storage: IpStaking CurrentEra (r:1 w:0)
	// Storage: IpStaking ContractEraStake (r:1 w:1)
	// Storage: IpStaking EraRewardsAndStakes (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn bond_and_stake() -> Weight {
		(373_299_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: IpStaking RegisteredIp (r:1 w:0)
	// Storage: IpStaking CurrentEra (r:1 w:0)
	// Storage: IpStaking ContractEraStake (r:3 w:1)
	// Storage: IpStaking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: IpStaking EraRewardsAndStakes (r:1 w:1)
	fn unbond_unstake_and_withdraw() -> Weight {
		(413_106_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: IpStaking RegisteredIp (r:1 w:0)
	// Storage: IpStaking CurrentEra (r:1 w:0)
	// Storage: IpStaking ContractEraStake (r:1 w:1)
	// Storage: IpStaking EraRewardsAndStakes (r:1 w:0)
	// Storage: Balances TotalIssuance (r:1 w:1)
	fn claim(n: u32, ) -> Weight {
		(76_835_000 as Weight)
			// Standard Error: 30_000
			.saturating_add((8_684_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: IpStaking ForceEra (r:0 w:1)
	fn force_new_era() -> Weight {
		(3_306_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: IpStaking RegisteredDevelopers (r:1 w:1)
	// Storage: IpStaking RegisteredIp (r:1 w:1)
	// Storage: IpStaking PreApprovalIsEnabled (r:1 w:0)
	fn register() -> Weight {
		(63_974_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: IpStaking RegisteredDevelopers (r:1 w:1)
	// Storage: IpStaking CurrentEra (r:1 w:0)
	// Storage: IpStaking ContractEraStake (r:3 w:0)
	// Storage: IpStaking EraRewardsAndStakes (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: IpStaking Ledger (r:25 w:25)
	// Storage: Balances Locks (r:25 w:25)
	fn unregister(n: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 68_000
			.saturating_add((43_004_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(n as Weight)))
	}
	// Storage: IpStaking PreApprovalIsEnabled (r:0 w:1)
	fn enable_developer_pre_approval() -> Weight {
		(3_132_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: IpStaking PreApprovedDevelopers (r:1 w:1)
	fn developer_pre_approval() -> Weight {
		(9_337_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: IpStaking RegisteredIp (r:1 w:0)
	// Storage: IpStaking RegisteredDevelopers (r:1 w:0)
	// Storage: IpStaking Ledger (r:1 w:1)
	// Storage: IpStaking CurrentEra (r:1 w:0)
	// Storage: IpStaking ContractEraStake (r:1 w:1)
	// Storage: IpStaking EraRewardsAndStakes (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn bond_and_stake() -> Weight {
		(373_299_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: IpStaking RegisteredIp (r:1 w:0)
	// Storage: IpStaking CurrentEra (r:1 w:0)
	// Storage: IpStaking ContractEraStake (r:3 w:1)
	// Storage: IpStaking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: IpStaking EraRewardsAndStakes (r:1 w:1)
	fn unbond_unstake_and_withdraw() -> Weight {
		(413_106_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: IpStaking RegisteredIp (r:1 w:0)
	// Storage: IpStaking CurrentEra (r:1 w:0)
	// Storage: IpStaking ContractEraStake (r:1 w:1)
	// Storage: IpStaking EraRewardsAndStakes (r:1 w:0)
	// Storage: Balances TotalIssuance (r:1 w:1)
	fn claim(n: u32, ) -> Weight {
		(76_835_000 as Weight)
			// Standard Error: 30_000
			.saturating_add((8_684_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: IpStaking ForceEra (r:0 w:1)
	fn force_new_era() -> Weight {
		(3_306_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}