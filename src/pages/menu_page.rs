use std::{    
    io::{self, Write},
};

pub struct Menu {
    
}

impl Menu {
    fn title<S: AsRef<str>>(string: S) -> String {
        format!("\n=== {} ===\n", string.as_ref())
    }

    //file commands should be in a seperate area
    fn save_file() {
        println!("Saving file...");
        // should call a method of file managment struct to save file
    }

    fn load_file() {
        println!("Loading file...");
        // should call a method of file managment struct to load file
    }

    //Task Editiing/Viewing
    fn create_new_task() {
        println!("Creating new task...");
        // should call a struct that manages tasks
    }

    fn edit_task() {
        println!("Editing task...");
        // should call a struct that manages tasks
    }

    fn view_task() {
        println!("Viewing task...");
        // should call a struct that manages tasks
    }

    fn display_menu_commands() {
        //write buffer
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        // === File Commands ===
        write!(handle, "{}", Self::title("File Commands"))
            .expect("Writing to stdout buffer failed.");

        //commands
        println!("0. Save file");
        println!("1. Load file");

        // === To-do List Commands ===
        write!(handle, "{}", Self::title("To-do List Commands"))
            .expect("Writing to stdout buffer failed.");

        //commands
        println!("2. Create New Task");
        println!("3. Edit Task");
        println!("4. View Task");
    }

    pub fn get_commands(&self) -> Vec<fn()> {
        // commands that are used to call the operations of Menu
        let mut vec: Vec<fn()> = Vec::new();
        vec.push(Self::save_file);
        vec.push(Self::load_file);
        vec.push(Self::create_new_task);
        vec.push(Self::edit_task);
        vec.push(Self::view_task);
        vec
    }

    pub fn render(&self) {
        print!("{}", Self::title("To-do List Terminal"));
        Self::display_menu_commands();
    }
}
