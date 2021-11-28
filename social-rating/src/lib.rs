use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::serde::Serialize;

struct App;

impl App {
    pub fn new() -> Self {
        Self
    }

    /// Rate another user `other_user` on bahalf of the current user (`predecessor_account_id`)
    pub fn rate(&mut self, other_user: near_sdk::AccountId, vote: Vote) {
        assert!(vote >= 1 && vote <= 5, "score value must be in a range of [1; 5]");
        let current_user = near_sdk::env::predecessor_account_id();
        assert!(current_user != other_user, "you cannot rate for yourself");

        // 1. prevent double-rating
        // assert!()
        // 2. update score of the other user
        let other_user_score = Score {
            total_votes: 10,
            number_of_votes: 2,
        };
        near_sdk::env::storage_write(other_user.as_ref().as_bytes(), &other_user_score.try_to_vec().unwrap());

        // 3. update information about double-rating
    }

    pub fn get_score(&self, user: near_sdk::AccountId) -> Option<Score> {
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

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
struct Score {
    total_votes: u64,
    number_of_votes: u64,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn it_works() {
        let mut app = App::new();

        app.rate(near_sdk::AccountId::from_str("frol.near").unwrap(), 5);
        assert!(
            matches!(
                app.get_score(near_sdk::AccountId::from_str("frol.near").unwrap()),
                Some(Score { total_votes: 10, number_of_votes: 2 })
            )
        );
    }
}
