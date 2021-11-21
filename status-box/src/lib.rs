use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

type AccountId = String;
type Message = String;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct StatusBox {
    storage: std::collections::HashMap<AccountId, Message>,
}

#[near_bindgen]
impl StatusBox {
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
