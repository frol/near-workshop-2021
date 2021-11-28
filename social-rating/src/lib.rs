use near_sdk::{near_bindgen, PanicOnDefault};
use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::serde::Serialize;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault, Serialize)]
#[serde(crate = "near_sdk::serde")]
struct App;

#[near_bindgen]
impl App {
    #[init]
    pub fn new() -> Self {
        Self
    }

    /// Rate another user `other_user` on bahalf of the current user (`predecessor_account_id`)
    pub fn rate(&mut self, other_user: &near_sdk::AccountId, vote: Vote) {
        assert!(vote >= 1 && vote <= 5, "score value must be in a range of [1; 5]");
        let current_user = near_sdk::env::predecessor_account_id();
        assert!(&current_user != other_user, "you cannot rate for yourself");

        // 1. prevent double-rating
        // assert!()
        // 2. update score of the other user
        let previous_score: Score = self.get_score(&other_user).unwrap_or_default();
        let other_user_score = Score {
            total_votes: previous_score.total_votes.checked_add(u64::from(vote)).expect("the user has reached the max total votes"),
            number_of_votes: previous_score.number_of_votes.checked_add(1).expect("the user has reached the max value for number of votes"),
        };
        near_sdk::env::storage_write(other_user.as_ref().as_bytes(), &other_user_score.try_to_vec().unwrap());

        // 3. update information about double-rating
    }

    pub fn get_score(&self, user: &near_sdk::AccountId) -> Option<Score> {
        //self.scores.get(user)
        let raw_score: Vec<u8> = near_sdk::env::storage_read(user.as_ref().as_bytes())?;
        let score: Score = Score::try_from_slice(&raw_score).expect("invalid internal storage state; cannot read Score from the storage");
        Some(score)
    }
}

// struct Vote {
//     vote_value: u8
// }
type Vote = u8;

#[derive(Debug, Default, BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
struct Score {
    total_votes: u64,
    number_of_votes: u64,
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use std::str::FromStr;

    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};

    use super::*;

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .is_view(is_view)
            .build()
    }


    #[test]
    fn it_works() {
        let mut context = get_context(false);

        let mut app = App::new();

        matches::assert_matches!(
            app.get_score(&near_sdk::AccountId::from_str("frol.near").unwrap()),
            None
        );

        context.predecessor_account_id = "bob2.near".to_string();
        testing_env!(context.clone());
        println!("rate #1");
        app.rate(&near_sdk::AccountId::from_str("frol.near").unwrap(), 5);
 
        matches::assert_matches!(
            app.get_score(&near_sdk::AccountId::from_str("frol.near").unwrap()),
            Some(Score { total_votes: 5, number_of_votes: 1 })
        );
       
        context.predecessor_account_id = "alice3.near".to_string();
        testing_env!(context.clone());
        println!("rate #2");
        app.rate(&near_sdk::AccountId::from_str("frol.near").unwrap(), 4);

        matches::assert_matches!(
            app.get_score(&near_sdk::AccountId::from_str("frol.near").unwrap()),
            Some(Score { total_votes: 9, number_of_votes: 2 })
        );
    }
}
