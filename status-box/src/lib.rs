use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

type AccountId = String;
type Message = String;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, near_sdk::PanicOnDefault)]
pub struct StatusBox {
    // Read more: https://www.near-sdk.io/contract-structure/collections
    storage: near_sdk::collections::LookupMap<AccountId, Message>,
}

#[near_bindgen]
impl StatusBox {
    #[init]
    pub fn new() -> Self {
        Self {
            storage: near_sdk::collections::LookupMap::new(b"s"),
        }
    }

    pub fn set_message(&mut self, username: &AccountId, message: &Message) {
        self.storage.insert(&username, &message);
    }

    pub fn get_message(&self, username: &AccountId) -> Option<Message> {
        self.storage.get(username)
    }
}
