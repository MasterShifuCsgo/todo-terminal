mod page;
mod task;



use std::io::{self, Write};

fn ask_option() -> u32 {
    print!("Option: ");
    io::stdout().flush().expect("Failed to flush stdout.");

    let mut user_choice = String::new();
    io::stdin()
        .read_line(&mut user_choice)
        .expect("Failed to read line");

    user_choice
        .trim()
        .parse()
        .expect("Failed to parse user input.")
}

fn hold() {
    print!("Waiting for user input...");
    io::stdout().flush().expect("Failed to flush stdout.");

    let mut user_choice = String::new();
    let _ = io::stdin().read_line(&mut user_choice);
}

fn main() {
    println!("=== To-do List Terminal ===\n");

    let mut mainpage: page::Page = page::Page::default();
    loop {
        mainpage.render();
        let immutable_commands: Vec<fn(&page::Page)> = mainpage.get_immutable_commands();
        let mutable_commands: Vec<fn(&mut page::Page)> = mainpage.get_mutable_commands();

        match ask_option() {
            0 => immutable_commands[0](&mainpage),
            1 => mutable_commands[0](&mut mainpage),
            2 => mutable_commands[1](&mut mainpage),
            3 => mutable_commands[2](&mut mainpage),
            4 => {
                immutable_commands[1](&mainpage);
                println!("Waiting for user input...");
                hold();
            },
            _ => panic!("User choise failed to match an operation"),
        }
    }
}
