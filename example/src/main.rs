use inquire::Text;

type AccountId = String;
type Message = String;

fn main() {
    let mut storage = std::collections::HashMap::<AccountId, Message>::new();

    loop {
        let command = Text::new("Какую команду выполнить?").prompt().unwrap();
        match command.as_str() {
            "get" => {
                let username = Text::new("Какой пользователь?").prompt().unwrap();
                if let Some(message) = storage.get(&username) {
                    println!("Status message: {}", message);
                } else {
                    println!("There is no status message set for the user '{}' yet", username);
                }
            }
            "set" => {
                let username = Text::new("Какой пользователь?").prompt().unwrap();
                let message = Text::new("Какой соощение статуса?").prompt().unwrap();
                storage.insert(username, message);
            }
            _ => {
                println!("такой команды нет");
            }
        }
    }
}
