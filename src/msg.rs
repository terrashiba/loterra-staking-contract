use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Decimal, HumanAddr, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub admin: HumanAddr,
    pub cw20_token_addr: HumanAddr,
    pub reward_denom: String,
    pub unbonding_period: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    ////////////////////
    /// Owner's operations
    ///////////////////

    /// Update the global index
    UpdateGlobalIndex {},

    ////////////////////
    /// Staking operations
    ///////////////////

    /// Increase user staking balance
    /// Withdraw rewards to pending rewards
    /// Set current reward index to global index
    BondBalance { amount: Uint128 },
    /// Unbound user staking balance
    /// Withdraw rewards to pending rewards
    /// Set current reward index to global index
    UnbondBalance { amount: Uint128 },

    ////////////////////
    /// User's operations
    ///////////////////

    /// return the accrued reward in usdt to the user.
    ClaimRewards { recipient: Option<HumanAddr> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    State {},
    AccruedRewards {
        address: HumanAddr,
    },
    Holder {
        address: HumanAddr,
    },
    Holders {
        start_after: Option<HumanAddr>,
        limit: Option<u32>,
    },
    /// Not used to be called directly
    TransferFrom {
        owner: HumanAddr,
        recipient: HumanAddr,
        amount: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub admin: HumanAddr,
    pub cw20_token_addr: HumanAddr,
    pub reward_denom: String,
    pub unbonding_period: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
    pub global_index: Decimal,
    pub total_balance: Uint128,
    pub prev_reward_balance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AccruedRewardsResponse {
    pub rewards: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct HolderResponse {
    pub address: HumanAddr,
    pub balance: Uint128,
    pub index: Decimal,
    pub pending_rewards: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct HoldersResponse {
    pub holders: Vec<HolderResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}
