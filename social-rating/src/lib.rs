struct App;

impl App {
    pub fn new() -> Self {
        Self
    }

    pub fn add_user(&mut self, account_id: near_sdk::AccountId) {
        near_sdk::env::storage_write(b"qq", b"hello");
    }

    pub fn get_user(&mut self, account_id: near_sdk::AccountId) -> Option<Vec<u8>> {
        near_sdk::env::storage_read(b"qq")
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use std::convert::TryInto;
    use std::str::FromStr;

    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};
    
    use super::*;

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(near_sdk::AccountId::from_str("bob_near").unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn my_test() {
        let context = get_context(false);
        testing_env!(context);
        
        let mut app = App::new();

        app.add_user(near_sdk::AccountId::from_str("asd").unwrap());
        assert!(app.get_user(near_sdk::AccountId::from_str("asd").unwrap()).is_none());
    }
}
