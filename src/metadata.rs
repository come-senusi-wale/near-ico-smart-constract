use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct InvestorMetadata {
    pub amount_invest: u128,
    pub token_reward: u128, 
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MemberMetadata {
    pub role: String,
    pub token_reward_status: u8, 
    pub amount_of_token_rewarded: u128
}