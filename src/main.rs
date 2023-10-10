use inquire::Text;

fn main() {
    loop {
        let command = Text::new("TODO-RS>").prompt().unwrap_or("".to_owned());
        match command.as_str() {
            "quit" => break,
            _ => println!("Sus")
        }
    }
}
