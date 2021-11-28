struct App;

impl App {
    pub fn new() -> Self {
        Self
    }

    pub fn add_user(&mut self, account_id: near_sdk::AccountId) {
        near_sdk::env::set()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let app = App::new();
    }
}
