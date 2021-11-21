use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use near_sdk::serde::Serialize;

type AccountId = String;
type Message = String;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct StatusBox {
    storage: std::collections::HashMap<AccountId, Message>,
}

#[near_bindgen]
impl StatusBox {
    #[init]
    pub fn new() -> Self {
        Self {
            storage: Default::default(),
        }
    }

    pub fn set_message(&mut self, username: AccountId, message: Message) {
        self.storage.insert(username, message);
    }

    pub fn get_message(&self, username: AccountId) -> Option<Message> {
        self.storage.get(&username).cloned()
    }
}
