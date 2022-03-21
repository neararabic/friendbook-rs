use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct Writing {
  pub message: String,
  pub sender: String,
  pub receiver: String,
}
