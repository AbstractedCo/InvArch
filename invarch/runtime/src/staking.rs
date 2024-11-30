use crate::{
    Balance, Balances, BlockNumber, ExistentialDeposit, MessageQueue, Runtime, RuntimeEvent,
    RuntimeHoldReason, DAYS, UNIT,
};
use cumulus_primitives_core::AggregateMessageOrigin;
use frame_support::{parameter_types, PalletId};
use pallet_dao_staking::primitives::CustomAggregateMessageOrigin;
parameter_types! {
    pub const BlocksPerEra: BlockNumber = DAYS;
    pub const RegisterDeposit: Balance = 5000 * UNIT;
    pub const MaxStakersPerDao: u32 = 10000;
    // Temporarily dropping down from 50 to 5.
    pub const MinimumStakingAmount: Balance = 5 * UNIT;
    pub const MaxEraStakeValues: u32 = 5;
    pub const MaxUnlockingChunks: u32 = 5;
    pub const UnbondingPeriod: u32 = 28;
    pub const OcifStakingPot: PalletId = PalletId(*b"inv/stak");
    pub const RewardRatio: (u32, u32) = (60, 40);
    pub const StakeThresholdForActiveDao: Balance = 250_000 * UNIT;
    pub const MaxNameLength: u32 = 20;
    pub const MaxDescriptionLength: u32 = 300;
    pub const MaxImageUrlLength: u32 = 100;
    pub const UnregisterOrigin: CustomAggregateMessageOrigin<AggregateMessageOrigin> = CustomAggregateMessageOrigin::UnregisterMessageOrigin;
}

impl pallet_dao_staking::Config for Runtime {
    type OldCurrency = Balances;
    type Currency = Balances;
    type BlocksPerEra = BlocksPerEra;
    type RegisterDeposit = RegisterDeposit;
    type RuntimeEvent = RuntimeEvent;
    type MaxStakersPerDao = MaxStakersPerDao;
    type ExistentialDeposit = ExistentialDeposit;
    type PotId = OcifStakingPot;
    type MaxUnlocking = MaxUnlockingChunks;
    type UnbondingPeriod = UnbondingPeriod;
    type MinimumStakingAmount = MinimumStakingAmount;
    type MaxEraStakeValues = MaxEraStakeValues;
    type RewardRatio = RewardRatio;
    type StakeThresholdForActiveDao = StakeThresholdForActiveDao;
    type MaxNameLength = MaxNameLength;
    type MaxDescriptionLength = MaxDescriptionLength;
    type MaxImageUrlLength = MaxImageUrlLength;
    type WeightInfo = pallet_dao_staking::weights::SubstrateWeight<Runtime>;
    type StakingMessage = frame_support::traits::EnqueueWithOrigin<MessageQueue, UnregisterOrigin>;
    type WeightToFee = crate::WeightToFee;
    type OnUnbalanced = crate::DealWithFees;
    type RuntimeHoldReason = RuntimeHoldReason;
}
