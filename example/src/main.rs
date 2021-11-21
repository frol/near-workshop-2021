use inquire::Text;

type AccountId = String;
type Message = String;

struct StatusBox {
    storage: std::collections::HashMap<AccountId, Message>,
}

impl StatusBox {
    pub fn new() -> Self {
        Self {
            //storage: std::collections::HashMap<AccountId, Message>::new(),
            storage: Default::default(),
        }
    }

    pub fn set_message(&mut self, username: AccountId, message: Message) {
        self.storage.insert(username, message);
    }

    pub fn get_message(&self, username: &AccountId) -> Option<&Message> {
        self.storage.get(username)
    }
}

fn main() {
    let mut status_box = StatusBox::new();

    loop {
        let command = Text::new("Какую команду выполнить?").prompt().unwrap();
        match command.as_str() {
            "get" => {
                let username = Text::new("Какой пользователь?").prompt().unwrap();
                if let Some(message) = status_box.get_message(&username) {
                    println!("Статус: {}", message);
                } else {
                    println!("Статус ещё не установлен для пользователя '{}'", username);
                }
            }
            "set" => {
                let username = Text::new("Какой пользователь?").prompt().unwrap();
                let message = Text::new("Укажите статус:").prompt().unwrap();
                status_box.set_message(username, message);
            }
            _ => {
                println!("Такой команды нет");
            }
        }
        println!();
    }
}
