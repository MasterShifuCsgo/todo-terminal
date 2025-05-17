use crate::pages::menu_page::Menu;

use std::io::{self, Write};

pub enum Pages {
    MenuPage(Menu),
}

impl Pages {
    fn ask_option(user_choise: &mut String, options: &Vec<fn()>) {
        //user choise
        print!("\nOption: ");
        io::stdout()
            .flush()
            .expect("Failed to flush output buffer.");

        io::stdin()
            .read_line(user_choise)
            .expect("Reading Option failed");

        //check if user enterd a number withing the correct range, 0..range
        let choise_result: Result<usize, _> = user_choise.trim().parse();
        match choise_result {
            Ok(choise) => {
                if choise < options.len() {
                    options[choise](); // run the 'command' that displays the UI of the 'command'
                } else {
                    println!("Choice out of range.")
                }
            }
            _ => println!("Invalid input."),
        }
    }

    pub fn render(&self) {
        let options: Vec<fn()>;

        match &self {
            Pages::MenuPage(page) => {
                options = page.get_commands();
                page.render();
            }
        }

        let mut user_choise: String = String::new();
        Self::ask_option(&mut user_choise, &options);
    }
}
